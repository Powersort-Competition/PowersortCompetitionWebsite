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

export async function getUserID(email) {
    let ID = 0;
    // Stub request. We are interested in only passing the email to the backend.
    let userID = async (email) => {
        let userRequest = {
            user_id: 0,
            first_name: "",
            last_name: "",
            email: email,
        }

        return axios.post("/my_user_id", userRequest)
            .then((response) => {
                return response.data;
            })
            .catch((error) => {
                console.error(error);
            });
    }

    ID = await userID(email).then((data) => {
        return data;
    });

    return ID;
}