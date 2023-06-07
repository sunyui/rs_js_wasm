async function main() {
  const module = await import('../../pkg/index');
  module.hello_world();
  num = module.fib(30)
  console.log(num)
  arr = module.send_array_to_js()
  console.log(arr)

  obj = module.send_obj_to_js()
  console.log(obj)

  console.log(module.test_point())
}

main();