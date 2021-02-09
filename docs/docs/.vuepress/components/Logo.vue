<template>
  <div class="logo">
    <img
      width="250px"
      height="250px"
      src="https://raw.githubusercontent.com/robertcorponoi/graphics/master/godot-rust-helper/godot-rust-helper-logo.png"
      alt="Godot Rust CLI Logo"
    />
    <h1>Godot Rust CLI</h1>
    <p>Easily use Rust with Godot</p>
    <p class="warning"><span>Note:</span> Godot Rust CLI is below v1.0.0 and may contain bugs, please report any bugs as issues in the <a href="https://github.com/robertcorponoi/godot-rust-cli" target="_blank">GitHub repo</a></p>
    <a class="btn" href="/guide">Get Started →</a>
  </div>
</template>

<style>
.logo {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}

.logo h1 {
  font-size: 3rem;
  margin-bottom: 0;
  margin-top: 0;
}

.logo p {
  font-size: 1.6rem;
  line-height: 2rem;
  color: #6a8bad;
  margin-bottom: 0;
}

.logo span {
  font-weight: bold;
}

.logo .warning {
  font-size: 1.2rem;
}

.logo .btn {
  font-size: 1.1rem;
  background-color: #27aae2;
  color: #fff;
  border-radius: 0.2rem;
  padding: 1rem 2rem;
  margin-top: 2rem;
  margin-bottom: 1.5rem;
}
</style>