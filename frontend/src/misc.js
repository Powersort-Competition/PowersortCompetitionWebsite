export function getInputSize(inputArr) {
  inputArr = inputArr.split(",");

  return inputArr.length;
}

export function backendHealthCheck() {
  return fetch("https://psortcomp.shayandoust.me/ping")
    .then((response) => response.json())
    .then((data) => {
      return data;
    });
}
