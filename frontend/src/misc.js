import axios from "axios";

export function getInputSize(inputArr) {
  inputArr = inputArr.split(",");

  return inputArr.length;
}

// TODO: Replace this function with axios.
export function backendHealthCheck() {
  return fetch("https://psortcomp.shayandoust.me/ping")
    .then((response) => response.json())
    .then((data) => {
      return data;
    });
}

export function getEmailFromCookie()
{
    return $cookies.get("pscomp_oauth").email;
}

export function getUserID(email)
{
    let userRequest = {
        user_id: 0,
        first_name: "",
        last_name: "",
        email: email,
    }

    return axios.post("/my_user_id", userRequest)
}