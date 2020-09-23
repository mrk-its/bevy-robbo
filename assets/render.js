function createShader(gl, type, source) {
    var shader = gl.createShader(type);
    gl.shaderSource(shader, source);
    gl.compileShader(shader);
    var success = gl.getShaderParameter(shader, gl.COMPILE_STATUS);
    if (success) {
      return shader;
    }

    console.log(gl.getShaderInfoLog(shader));
    gl.deleteShader(shader);
  }
  function createProgram(gl, vertexShader, fragmentShader) {
    var program = gl.createProgram();
    gl.attachShader(program, vertexShader);
    gl.attachShader(program, fragmentShader);
    gl.linkProgram(program);
    var success = gl.getProgramParameter(program, gl.LINK_STATUS);
    if (success) {
      return program;
    }

    console.log(gl.getProgramInfoLog(program));
    gl.deleteProgram(program);
  }

  const canvas = document.querySelector("#bevy-canvas");
  const gl = canvas.getContext("webgl2", { alpha: false });
  const vertexShaderSource = `#version 300 es
    // an attribute is an input (in) to a vertex shader.
    // It will receive data from a buffer
    in vec2 a_position;
    in vec2 a_texcoord;

    uniform vec2 u_resolution;

    // a varying to pass the texture coordinates to the fragment shader
    out vec2 v_texcoord;

    // all shaders have a main function
    void main() {
      // convert the position from pixels to 0.0 to 1.0
      vec2 zeroToOne = a_position / u_resolution;
      vec2 zeroToTwo = zeroToOne * 2.0 ;
      vec2 clipSpace = zeroToTwo - 1.0;

      // gl_Position is a special variable a vertex shader
      // is responsible for setting
      gl_Position = vec4(clipSpace, 0, 1);
      v_texcoord = a_texcoord;
    }
  `;
  const fragmentShaderSource = `#version 300 es
    // fragment shaders don't have a default precision so we need
    // to pick one. highp is a good default. It means "high precision"
    precision highp float;

    // Passed in from the vertex shader.
    in vec2 v_texcoord;

    // The texture.
    uniform sampler2D u_texture;

    // we need to declare an output for the fragment shader
    out vec4 outColor;

    void main() {
      // Just set the output to a constant reddish-purple
      // outColor = vec4(1, 0, 0.5, 1);
      outColor = texture(u_texture, v_texcoord);
    }
    `;
  var vertexShader = createShader(gl, gl.VERTEX_SHADER, vertexShaderSource);
  var fragmentShader = createShader(gl, gl.FRAGMENT_SHADER, fragmentShaderSource);
  var program = createProgram(gl, vertexShader, fragmentShader);
  var positionAttributeLocation = gl.getAttribLocation(program, "a_position");
  var translationAttributeLocation = gl.getAttribLocation(program, "a_translation");
  var texcoordAttributeLocation = gl.getAttribLocation(program, "a_texcoord");

  var resolutionUniformLocation = gl.getUniformLocation(program, "u_resolution");

  var positionBuffer = gl.createBuffer();
  gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);
  // three 2d points
  var vao = gl.createVertexArray();
  gl.bindVertexArray(vao);
  gl.enableVertexAttribArray(positionAttributeLocation);
  var size = 2;          // 2 components per iteration
  var type = gl.FLOAT;   // the data is 32bit floats
  var normalize = false; // don't normalize the data
  var stride = 16;        // 0 = move forward size * sizeof(type) each iteration to get the next position
  var offset = 0;        // start at the beginning of the buffer
  gl.vertexAttribPointer(
    positionAttributeLocation, size, type, normalize, stride, offset)


  function create_tile(positions, index, x, y, tile_x, tile_y) {
    const tile = [
      x, y, tile_x / 12, 1 - tile_y / 8,
      x, y + 32, tile_x / 12.0, 1 - (tile_y + 1) / 8,
      x + 32, y, (tile_x + 1) / 12, 1 - tile_y / 8,
      x + 32, y + 32, (tile_x + 1) / 12, 1 - (tile_y + 1) / 8,
      x + 32, y, (tile_x + 1) / 12, 1 - tile_y / 8,
      x, y + 32, tile_x / 12, 1 - (tile_y + 1) / 8,
    ];
    positions.set(tile, index);
    return index + tile.length;
  }


  gl.enableVertexAttribArray(texcoordAttributeLocation);
  gl.vertexAttribPointer(texcoordAttributeLocation, 2, gl.FLOAT, true, 16, 8);

  var texture = gl.createTexture();
  gl.bindTexture(gl.TEXTURE_2D, texture);
  // gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, 2, 2, 0, gl.RGBA, gl.UNSIGNED_BYTE,
  //   new Uint8Array([0, 0, 255, 255, 0, 255, 0, 255, 255, 0, 0, 255, 255, 255, 255, 255]));
  gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, 1, 1, 0, gl.RGBA, gl.UNSIGNED_BYTE,
    new Uint8Array([0, 0, 255, 255, 0, 255, 255, 255]));

  var image = new Image();
  image.src = "assets/icons32.png";
  image.addEventListener('load', function () {
    // Now that the image has loaded make copy it to the texture.
    gl.bindTexture(gl.TEXTURE_2D, texture);
    gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, gl.RGBA, gl.UNSIGNED_BYTE, image);
    gl.generateMipmap(gl.TEXTURE_2D);
  });

  gl.viewport(0, 0, gl.canvas.width, gl.canvas.height);
  var t = 0;


  var buffer = new ArrayBuffer(31 * 16 * 4 * 6 * 4);

  var positions = new Float32Array(buffer);
  gl.enable(gl.BLEND);
  gl.blendFunc(gl.ONE, gl.ONE_MINUS_SRC_ALPHA);


export function render(screws, keys, ammo, level, board) {
    document.getElementById("inventory").innerText=`screws: ${screws}, keys: ${keys}, ammo: ${ammo}, level: ${level}`;
    // document.getElementById("board").innerText = board;
    var start_index = 0;
    for (var i = 0; i < board.length; i++) {
        let tile = board[i];
        if(tile >= 255) continue;
        const x = i % 31;
        const y = 15 - Math.floor(i / 31);
        const tile_x =  tile % 12;
        const tile_y =  (7-Math.floor(tile / 12));
        start_index = create_tile(positions, start_index, x * 32, 64 + y * 32, tile_x, tile_y);
    }
    gl.bufferData(gl.ARRAY_BUFFER, positions, gl.STATIC_DRAW);
    gl.clearColor(0.2, 0.2, 0.2, 1.0);

    gl.disable(gl.DEPTH_TEST);

    gl.clear(gl.COLOR_BUFFER_BIT);

    gl.useProgram(program);
    gl.uniform2f(resolutionUniformLocation, gl.canvas.width, gl.canvas.height);

    gl.bindVertexArray(vao);

    var primitiveType = gl.TRIANGLES
    var offset = 0;
    var count = start_index / 4;
    gl.drawArrays(primitiveType, offset, count);
}
