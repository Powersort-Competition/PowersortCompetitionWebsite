import axios from "axios";

export function getInputSize(inputArr) {
    inputArr = inputArr.split(",");

    return inputArr.length;
}

// Function to round num to three decimal places.
function roundToThree(num) {
    return +(Math.round(num + "e+3") + "e-3");
}

// Work out the (absolute) difference between the two sort merge costs.
export function percDifference(tsortMergeCost, psortMergeCost)
{
    let diff = (100 *
        Math.abs(tsortMergeCost - psortMergeCost) / ((tsortMergeCost + psortMergeCost) / 2));

    return roundToThree(diff);
}

// TODO: Replace this function with axios.
export function backendHealthCheck() {
    // return fetch("https://psortcomp.shayandoust.me/ping")
    //     .then((response) => response.json())
    //     .then((data) => {
    //         return data;
    //     });
    return axios.get("/ping")
        .then((response) => {
            return response.data;
        })
        .catch((error) => {
            return null;
        });
}

export function getEmailFromCookie() {
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