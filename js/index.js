const run = async () => {
  const $s = document.getElementById("state");

  $s.innerText = "Loading...";

  try {
    const mod = await import("../pkg/index.js");

    $s.innerText = "Running...";
    await mod.run_app();
  } catch (err) {
    if (!err.message.includes("Using exceptions for control flow")) {
      console.error(err);
      $s.innerText = "Error!";
      return;
    }
  }

  $s.innerText = "";
};
run();
