/* C++ code :
template<typename T, qualifier Q>
GLM_FUNC_QUALIFIER bool decompose(mat<4, 4, T, Q> const& ModelMatrix, vec<3, T, Q> & Scale, qua<T, Q> & Orientation, vec<3, T, Q> & Translation, vec<3, T, Q> & Skew, vec<4, T, Q> & Perspective)
{
mat<4, 4, T, Q> LocalMatrix(ModelMatrix);

// Normalize the matrix.
if(epsilonEqual(LocalMatrix[3][3], static_cast<T>(0), epsilon<T>()))
return false;

for(length_t i = 0; i < 4; ++i)
for(length_t j = 0; j < 4; ++j)
LocalMatrix[i][j] /= LocalMatrix[3][3];

// perspectiveMatrix is used to solve for perspective, but it also provides
// an easy way to test for singularity of the upper 3x3 component.
mat<4, 4, T, Q> PerspectiveMatrix(LocalMatrix);

for(length_t i = 0; i < 3; i++)
PerspectiveMatrix[i][3] = static_cast<T>(0);
PerspectiveMatrix[3][3] = static_cast<T>(1);

/// TODO: Fixme!
if(epsilonEqual(determinant(PerspectiveMatrix), static_cast<T>(0), epsilon<T>()))
return false;

// First, isolate perspective.  This is the messiest.
if(
epsilonNotEqual(LocalMatrix[0][3], static_cast<T>(0), epsilon<T>()) ||
epsilonNotEqual(LocalMatrix[1][3], static_cast<T>(0), epsilon<T>()) ||
epsilonNotEqual(LocalMatrix[2][3], static_cast<T>(0), epsilon<T>()))
{
// rightHandSide is the right hand side of the equation.
vec<4, T, Q> RightHandSide;
RightHandSide[0] = LocalMatrix[0][3];
RightHandSide[1] = LocalMatrix[1][3];
RightHandSide[2] = LocalMatrix[2][3];
RightHandSide[3] = LocalMatrix[3][3];

// Solve the equation by inverting PerspectiveMatrix and multiplying
// rightHandSide by the inverse.  (This is the easiest way, not
// necessarily the best.)
mat<4, 4, T, Q> InversePerspectiveMatrix = glm::inverse(PerspectiveMatrix);//   inverse(PerspectiveMatrix, inversePerspectiveMatrix);
mat<4, 4, T, Q> TransposedInversePerspectiveMatrix = glm::transpose(InversePerspectiveMatrix);//   transposeMatrix4(inversePerspectiveMatrix, transposedInversePerspectiveMatrix);

Perspective = TransposedInversePerspectiveMatrix * RightHandSide;
//  v4MulPointByMatrix(rightHandSide, transposedInversePerspectiveMatrix, perspectivePoint);

// Clear the perspective partition
LocalMatrix[0][3] = LocalMatrix[1][3] = LocalMatrix[2][3] = static_cast<T>(0);
LocalMatrix[3][3] = static_cast<T>(1);
}
else
{
// No perspective.
Perspective = vec<4, T, Q>(0, 0, 0, 1);
}

// Next take care of translation (easy).
Translation = vec<3, T, Q>(LocalMatrix[3]);
LocalMatrix[3] = vec<4, T, Q>(0, 0, 0, LocalMatrix[3].w);

vec<3, T, Q> Row[3], Pdum3;

// Now get scale and shear.
for(length_t i = 0; i < 3; ++i)
for(length_t j = 0; j < 3; ++j)
Row[i][j] = LocalMatrix[i][j];

// Compute X scale factor and normalize first row.
Scale.x = length(Row[0]);// v3Length(Row[0]);

Row[0] = detail::scale(Row[0], static_cast<T>(1));

// Compute XY shear factor and make 2nd row orthogonal to 1st.
Skew.z = dot(Row[0], Row[1]);
Row[1] = detail::combine(Row[1], Row[0], static_cast<T>(1), -Skew.z);

// Now, compute Y scale and normalize 2nd row.
Scale.y = length(Row[1]);
Row[1] = detail::scale(Row[1], static_cast<T>(1));
Skew.z /= Scale.y;

// Compute XZ and YZ shears, orthogonalize 3rd row.
Skew.y = glm::dot(Row[0], Row[2]);
Row[2] = detail::combine(Row[2], Row[0], static_cast<T>(1), -Skew.y);
Skew.x = glm::dot(Row[1], Row[2]);
Row[2] = detail::combine(Row[2], Row[1], static_cast<T>(1), -Skew.x);

// Next, get Z scale and normalize 3rd row.
Scale.z = length(Row[2]);
Row[2] = detail::scale(Row[2], static_cast<T>(1));
Skew.y /= Scale.z;
Skew.x /= Scale.z;

// At this point, the matrix (in rows[]) is orthonormal.
// Check for a coordinate system flip.  If the determinant
// is -1, then negate the matrix and the scaling factors.
Pdum3 = cross(Row[1], Row[2]); // v3Cross(row[1], row[2], Pdum3);
if(dot(Row[0], Pdum3) < 0)
{
for(length_t i = 0; i < 3; i++)
{
Scale[i] *= static_cast<T>(-1);
Row[i] *= static_cast<T>(-1);
}
}

// Now, get the rotations out, as described in the gem.

// FIXME - Add the ability to return either quaternions (which are
// easier to recompose with) or Euler angles (rx, ry, rz), which
// are easier for authors to deal with. The latter will only be useful
// when we fix https://bugs.webkit.org/show_bug.cgi?id=23799, so I
// will leave the Euler angle code here for now.

// ret.rotateY = asin(-Row[0][2]);
// if (cos(ret.rotateY) != 0) {
//     ret.rotateX = atan2(Row[1][2], Row[2][2]);
//     ret.rotateZ = atan2(Row[0][1], Row[0][0]);
// } else {
//     ret.rotateX = atan2(-Row[2][0], Row[1][1]);
//     ret.rotateZ = 0;
// }

int i, j, k = 0;
T root, trace = Row[0].x + Row[1].y + Row[2].z;
if(trace > static_cast<T>(0))
{
root = sqrt(trace + static_cast<T>(1.0));
Orientation.w = static_cast<T>(0.5) * root;
root = static_cast<T>(0.5) / root;
Orientation.x = root * (Row[1].z - Row[2].y);
Orientation.y = root * (Row[2].x - Row[0].z);
Orientation.z = root * (Row[0].y - Row[1].x);
} // End if > 0
else
{
static int Next[3] = {1, 2, 0};
i = 0;
if(Row[1].y > Row[0].x) i = 1;
if(Row[2].z > Row[i][i]) i = 2;
j = Next[i];
k = Next[j];

#           ifdef GLM_FORCE_QUAT_DATA_WXYZ
int off = 1;
#           else
int off = 0;
#           endif

root = sqrt(Row[i][i] - Row[j][j] - Row[k][k] + static_cast<T>(1.0));

Orientation[i + off] = static_cast<T>(0.5) * root;
root = static_cast<T>(0.5) / root;
Orientation[j + off] = root * (Row[i][j] + Row[j][i]);
Orientation[k + off] = root * (Row[i][k] + Row[k][i]);
Orientation.w = root * (Row[j][k] - Row[k][j]);
} // End if <= 0

return true;
}*/
use glam::*;
use std::ops::Index;
/// Decompose a matrix into its scale, orientation, translation, skew and perspective components.
/// From https://github.com/g-truc/glm/blob/master/glm/gtx/matrix_decompose.inl
pub fn decompose(mut matrix: Mat4) -> (Vec3, Quat, Vec3, Vec3, Vec4) {
    let mut scale = Vec3::ZERO;
    let mut orientation = Quat::IDENTITY;
    let mut translation = Vec3::ZERO;
    let mut skew = Vec3::ZERO;
    let mut perspective = Vec4::ZERO;

    // Normalize the matrix.
    if matrix.w_axis.w == 0.0 {
        panic!("Matrix is not normalized");
        return (
            Vec3::ZERO,
            Quat::IDENTITY,
            Vec3::ZERO,
            Vec3::ZERO,
            Vec4::ZERO,
        );
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
        return (
            Vec3::ZERO,
            Quat::IDENTITY,
            Vec3::ZERO,
            Vec3::ZERO,
            Vec4::ZERO,
        );
    }

    // First, isolate perspective.  This is the messiest.
    if matrix.x_axis.w != 0.0 || matrix.y_axis.w != 0.0 || matrix.z_axis.w != 0.0 {
        // rightHandSide is the right hand side of the equation.
        let right_hand_side = Vec4::new(
            matrix.x_axis.w,
            matrix.y_axis.w,
            matrix.z_axis.w,
            matrix.w_axis.w,
        );

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
    use std::f32::consts::PI;
    use super::*;
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
        let proj = Mat4::perspective_lh(45.0_f32.to_radians(), 4.0/3.0, 0.1, 100.0);
        let cam = Mat4::look_at_rh(Vec3::new(4.0, 3.0, 3.0), Vec3::ZERO, Vec3::Y);
        let orient = Mat4::from_scale(Vec3::new(-1.0, 1.0, 1.0));
        let matrix = proj * cam * orient;
        let (scale2, orientation2, translation2, skew2, perspective2) = decompose(matrix);

        assert!(vec3_close(Vec3::new(-0.278659, -0.318692, -0.248781), scale2));
        assert!(quat_close(Quat::from_xyzw(0.298198, 0.196601, 0.891995, 0.277076), orientation2));
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
        (a.x - b.x).abs() < epsilon && (a.y - b.y).abs() < epsilon && (a.z - b.z).abs() < epsilon && (a.w - b.w).abs() < epsilon
    }
    
    fn vec4_close(a: Vec4, b: Vec4) -> bool {
        let epsilon = 0.0001;
        (a.x - b.x).abs() < epsilon && (a.y - b.y).abs() < epsilon && (a.z - b.z).abs() < epsilon && (a.w - b.w).abs() < epsilon
    }
}