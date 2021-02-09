<template>
  <div class="features">
    <div>
      <h4>Easy to Use</h4>
      <p>With a few basic commands you can setup and use Rust modules with your Godot project.</p>
    </div>
    <div>
      <h4>Manage Modules</h4>
      <p>Godot Rust CLI creates and destroys modules for you so you don't have to worry about managing modules.</p>
    </div>
    <div>
      <h4>Create Plugins</h4>
      <p>Creating plugins is as easy as creating any other Rust module.</p>
    </div>
  </div>
</template>

<style>
.features {
  display: grid;
  grid-template-columns: 1fr;
  column-gap: 2rem;
}

@media only screen and (min-width: 768px) {
  .features {
    grid-template-columns: 1fr 1fr 1fr;
  }
}

.features h4 {
  font-size: 1.25rem;
  color: #3e556b;
  margin-bottom: 0.2rem;
}

.features p {
  color: #6a8bad;
}
</style>