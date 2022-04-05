use super::*;

pub fn render(
    mesh: &Mesh, 
    shader: &Shader,
    particle: &particle::Particle,
    vp: &Matrix4<f32>
) {
    let color = particle.color;
    let size = particle.size;
    let position = particle.position;

    shader.bind();

    let mut model = Matrix4::<f32>::identity();
    model = model * Matrix4::from_translation(position.clone());
    model.x.x *= size.x;  // scale x
    model.y.y *= size.y;  // scale y

    let mvp: Matrix4<f32> = vp * model;

    // Set uniforms 
    shader.set("uMVP", shader::SetType::Mat4(&mvp)); 
    shader.set("uColor", shader::SetType::Vec4(&color));
    
    unsafe {
        // Bind the mesh
        gl::BindVertexArray(mesh.vao);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, mesh.ebo);

        // Draw the quad
        gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const _);

        // Unbind the mesh
        gl::BindVertexArray(0);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
    }

    shader.unbind();
}