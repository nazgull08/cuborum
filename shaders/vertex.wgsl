struct VertexInput {
    @location(0) position: vec3<f32>, // Входные координаты
    @location(1) color: vec4<f32>,    // Входной цвет
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>, // Координаты вершины после трансформации
    @location(0) color: vec4<f32>,          // Передача цвета во фрагментный шейдер
};

@vertex
fn main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.position = vec4<f32>(input.position, 1.0); // Без матрицы пока
    output.color = input.color;
    return output;
}
