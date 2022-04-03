use std::mem;

pub struct Mesh {
    pub vao: u32,
    pub vbo: u32,
    pub ebo: u32,
}

impl Mesh {
    pub fn new() -> Mesh {

        // Vertex data
        let vertex_buffer_: [f32; 16] = [
            // position         // index
            0.0, 0.0, 1.0,      0.0,
            1.0, 0.0, 1.0,      1.0,
            1.0, 1.0, 1.0,      2.0,
            0.0, 1.0, 1.0,      3.0,
        ];
        let vertex_size: usize = 4 * mem::size_of::<f32>(); 

        // Index data
        let index_buffer_: [u32; 6] = [
            0, 1, 2,
            2, 3, 0
        ];

        // Mesh
        let mut mesh = Mesh{ vao: 0, vbo: 0, ebo: 0 };

        unsafe {
            // Create VAO
            gl::GenVertexArrays(1, &mut mesh.vao);
            gl::BindVertexArray(mesh.vao);

            // Create VBO and send initial data, this data on the GPU won't be changed.
            gl::GenBuffers(1, &mut mesh.vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, mesh.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER, 
                mem::size_of_val(&vertex_buffer_) as isize,
                vertex_buffer_.as_ptr().cast(),
                gl::STATIC_DRAW
            );

            gl::VertexAttribPointer(0, 3,
                gl::FLOAT, gl::FALSE,
                vertex_size as i32, core::ptr::null()
            );
            gl::EnableVertexAttribArray(0);

            gl::VertexAttribPointer(1, 1,
                gl::FLOAT, gl::FALSE,
                vertex_size as i32, (3 * mem::size_of::<f32>()) as *const _
            );
            gl::EnableVertexAttribArray(1);

            // Create EBO and send initial data, this data on the GPU won't be changed.
            gl::GenBuffers(1, &mut mesh.ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, mesh.ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER, 
                mem::size_of_val(&index_buffer_) as isize,
                index_buffer_.as_ptr().cast(),
                gl::STATIC_DRAW
            );

            // Unbind Objects
            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }

        // Return mesh
        mesh
    }
}