// eslint-disable-next-line import/extensions
import("./index").catch((e) =>
  // eslint-disable-next-line no-console
  {
    console.error("Error importing index.js :", e);
  },
);
