export function simple() {
  const cond = false;
  let t = 0;

  function main(x) {
    if (cond) {
      sideeffect(1);
    }
    if (t) {
      sideeffect(2);
    }
    t++;
    return x > 100 ? x : main(2 * x + 1);
  }

  return main(1);
}

export function complex1() {
  function main(a) {
    return () => g(a)
  }

  function g(a) {
    return () => main(a + 1)
  }

  t = main(1)
}

export function complex2() {
  function main() {
    enterHooks = resolveTransitionHooks(
      (hooks) => enterHooks = hooks
    );
  }
  function resolveTransitionHooks(postClone) {
    const hooks = {
      clone() {
        const hooks2 = resolveTransitionHooks(postClone);
        if (postClone) postClone(hooks2);
        return hooks2;
      }
    };
    return hooks;
  }
  return main;
}