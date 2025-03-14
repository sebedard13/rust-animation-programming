use glam::*;

/// Decompose a matrix into its scale, orientation, translation, skew and perspective components.
/// From https://github.com/g-truc/glm/blob/master/glm/gtx/matrix_decompose.inl
pub fn decompose(mut matrix: Mat4) -> (Vec3, Quat, Vec3, Vec3, Vec4) {
    let mut scale = Vec3::ZERO;
    let mut orientation = Quat::IDENTITY;
    let translation;
    let mut skew = Vec3::ZERO;
    let perspective;

    // Normalize the matrix.
    if matrix.w_axis.w == 0.0 {
        panic!("Matrix is not normalized");
    }

    for i in 0..4 {
        for j in 0..4 {
            let w = matrix.w_axis.w;
            let value = index_in_mat(&mut matrix, i);
            value[j] /= w;
        }
    }

    // perspectiveMatrix is used to solve for perspective, but it also provides
    // an easy way to test for singularity of the upper 3x3 component.
    let mut perspective_matrix = matrix;

    perspective_matrix.x_axis.w = 0.0;
    perspective_matrix.y_axis.w = 0.0;
    perspective_matrix.z_axis.w = 0.0;
    perspective_matrix.w_axis[3] = 1.0;

    if perspective_matrix.determinant() == 0.0 {
        return (Vec3::ZERO, Quat::IDENTITY, Vec3::ZERO, Vec3::ZERO, Vec4::ZERO);
    }

    // First, isolate perspective.  This is the messiest.
    if matrix.x_axis.w != 0.0 || matrix.y_axis.w != 0.0 || matrix.z_axis.w != 0.0 {
        // rightHandSide is the right hand side of the equation.
        let right_hand_side = Vec4::new(matrix.x_axis.w, matrix.y_axis.w, matrix.z_axis.w, matrix.w_axis.w);

        // Solve the equation by inverting PerspectiveMatrix and multiplying
        // rightHandSide by the inverse.  (This is the easiest way, not
        // necessarily the best.)
        let inverse_perspective_matrix = perspective_matrix.inverse();
        let transposed_inverse_perspective_matrix = inverse_perspective_matrix.transpose();

        perspective = transposed_inverse_perspective_matrix * right_hand_side;

        // Clear the perspective partition

        matrix.x_axis.w = 0.0;
        matrix.y_axis.w = 0.0;
        matrix.z_axis.w = 0.0;
        matrix.w_axis.w = 1.0;
    } else {
        // No perspective.
        perspective = Vec4::new(0.0, 0.0, 0.0, 1.0);
    }

    translation = matrix.w_axis.truncate();
    matrix.w_axis.x = 0.0;
    matrix.w_axis.y = 0.0;
    matrix.w_axis.z = 0.0;
    matrix.w_axis.w = matrix.w_axis.w;

    let mut row = [Vec3::ZERO; 3];

    // Now get scale and shear.
    for i in 0..3 {
        for j in 0..3 {
            let value = index_in_mat(&mut matrix, i);
            row[i][j] = value[j];
        }
    }

    // Compute X scale factor and normalize first row.
    scale.x = row[0].length();
    row[0] = row[0].normalize();

    // Compute XY shear factor and make 2nd row orthogonal to 1st.
    skew.z = row[0].dot(row[1]);
    row[1] = combine(row[1], row[0], 1.0, -skew.z);

    // Now, compute Y scale and normalize 2nd row.
    scale.y = row[1].length();
    row[1] = row[1].normalize();
    skew.z /= scale.y;

    // Compute XZ and YZ shears, orthogonalize 3rd row.
    skew.y = row[0].dot(row[2]);
    row[2] = combine(row[2], row[0], 1.0, -skew.y);
    skew.x = row[1].dot(row[2]);
    row[2] = combine(row[2], row[1], 1.0, -skew.x);

    // Next, get Z scale and normalize 3rd row.
    scale.z = row[2].length();
    row[2] = row[2].normalize();
    skew.y /= scale.z;
    skew.x /= scale.z;

    // At this point, the matrix (in rows[]) is orthonormal.
    // Check for a coordinate system flip.  If the determinant

    let pdum3 = row[1].cross(row[2]);
    let res = row[0].dot(pdum3);
    if res < 0.0 {
        for i in 0..3 {
            scale[i] *= -1.0;
            row[i] *= -1.0;
        }
    }

    // Now, get the rotations out, as described in the gem.

    let trace = row[0].x + row[1].y + row[2].z;
    if trace > 0.0 {
        let mut root = (trace + 1.0).sqrt();
        orientation.w = 0.5 * root;
        root = 0.5 / root;
        orientation.x = root * (row[1].z - row[2].y);
        orientation.y = root * (row[2].x - row[0].z);
        orientation.z = root * (row[0].y - row[1].x);
    } else {
        let next = vec![1, 2, 0];
        let mut i = 0;
        if row[1].y > row[0].x {
            i = 1;
        }
        if row[2].z > row[i][i] {
            i = 2;
        }
        let j = next[i];
        let k = next[j];

        let mut root = (row[i][i] - row[j][j] - row[k][k] + 1.0).sqrt();

        let orientation_i = index_in_quat(&mut orientation, i);
        *orientation_i = 0.5 * root;
        root = 0.5 / root;
        let orientation_j = index_in_quat(&mut orientation, j);
        *orientation_j = root * (row[i][j] + row[j][i]);
        let orientation_k = index_in_quat(&mut orientation, k);
        *orientation_k = root * (row[i][k] + row[k][i]);
        orientation.w = root * (row[j][k] - row[k][j]);
    }

    (scale, orientation, translation, skew, perspective)
}

fn index_in_quat(quat: &mut Quat, index: usize) -> &mut f32 {
    match index {
        0 => &mut quat.x,
        1 => &mut quat.y,
        2 => &mut quat.z,
        3 => &mut quat.w,
        _ => panic!("Invalid index for quaternion"),
    }
}

fn index_in_mat(mat: &mut Mat4, index: usize) -> &mut Vec4 {
    match index {
        0 => &mut mat.x_axis,
        1 => &mut mat.y_axis,
        2 => &mut mat.z_axis,
        3 => &mut mat.w_axis,
        _ => panic!("Invalid index for matrix"),
    }
}

fn combine(a: Vec3, b: Vec3, t1: f32, t2: f32) -> Vec3 {
    (a * t1) + (b * t2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;
    #[test]
    fn matrix_created() {
        let scale = Vec3::new(2.0, 0.2, 1.0);
        let orientation = Quat::from_euler(EulerRot::XYZ, 0.5 * PI, 0.0, -0.5 * PI);
        let translation = Vec3::new(1.0, 0.0, -1.0);
        let skew = Vec3::new(0.0, 0.0, 0.0);
        let perspective = Vec4::new(0.0, 0.0, 0.0, 1.0);

        let matrix = Mat4::from_scale_rotation_translation(scale, orientation, translation);
        let (scale2, orientation2, translation2, skew2, perspective2) = decompose(matrix);

        assert!(vec3_close(scale, scale2));
        assert!(quat_close(orientation, orientation2));
        assert!(vec3_close(translation, translation2));
        assert!(vec3_close(skew, skew2));
        assert!(vec4_close(perspective, perspective2));
    }

    #[test]
    #[ignore]
    fn perspective_matrix() {
        let proj = Mat4::perspective_lh(45.0_f32.to_radians(), 4.0 / 3.0, 0.1, 100.0);
        let cam = Mat4::look_at_rh(Vec3::new(4.0, 3.0, 3.0), Vec3::ZERO, Vec3::Y);
        let orient = Mat4::from_scale(Vec3::new(-1.0, 1.0, 1.0));
        let matrix = proj * cam * orient;
        let (scale2, orientation2, translation2, skew2, perspective2) = decompose(matrix);

        assert!(vec3_close(Vec3::new(-0.278659, -0.318692, -0.248781), scale2));
        assert!(quat_close(
            Quat::from_xyzw(0.298198, 0.196601, 0.891995, 0.277076),
            orientation2
        ));
        assert!(vec3_close(Vec3::new(0.0, 0.0, 0.967668), translation2));
        assert!(vec3_close(Vec3::new(-0.588286, -0.20312, -0.563927), skew2));
        assert!(vec4_close(Vec4::new(0.0, 0.0, 0.998002, 0.0342655), perspective2));
    }

    fn vec3_close(a: Vec3, b: Vec3) -> bool {
        let epsilon = 0.0001;
        (a.x - b.x).abs() < epsilon && (a.y - b.y).abs() < epsilon && (a.z - b.z).abs() < epsilon
    }

    fn quat_close(a: Quat, b: Quat) -> bool {
        let epsilon = 0.0001;
        (a.x - b.x).abs() < epsilon
            && (a.y - b.y).abs() < epsilon
            && (a.z - b.z).abs() < epsilon
            && (a.w - b.w).abs() < epsilon
    }

    fn vec4_close(a: Vec4, b: Vec4) -> bool {
        let epsilon = 0.0001;
        (a.x - b.x).abs() < epsilon
            && (a.y - b.y).abs() < epsilon
            && (a.z - b.z).abs() < epsilon
            && (a.w - b.w).abs() < epsilon
    }
}
