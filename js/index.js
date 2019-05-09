import("../crate/pkg")
  .then(module => {
    return module.run();
  })
  .then(res => console.log(res))
  .catch(err => console.error(err));
