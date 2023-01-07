function writable(init = {}) {
  let _val = init;
  // const subs = [];
  const subs = new Map();

  const subscribe = (topic: string, cb: any) => {
    const topicSubs = subs.get(topic) || [];
    subs.set(topic, [cb].concat(topicSubs));
    cb(_val);

    return () => {
      const topicSubs = subs.get(topic) || [];
      const index = topicSubs.findIndex((fn: any) => fn === cb);
      topicSubs.splice(index, 1);
      subs.set(topic, topicSubs);
    };
  };

  const set = (topic: string, v: any) => {
    _val = v;
    const topicSubs = subs.get(topic) || [];
    topicSubs.forEach((fn: any) => fn(_val));
  };

  const update = (topic: string, fn: any) => set(topic, fn(_val));

  return { subscribe, set, update };
}
