import axios from "axios";

export function getInputSize(inputArr) {
    inputArr = inputArr.split(",");

    return inputArr.length;
}

// Function to round num to three decimal places.
function roundToThree(num) {
    return +(Math.round(num + "e+3") + "e-3");
}

// Work out the (absolute) difference between the a and b.
export function percDifference(a, b) {
    if (a === 0 && b === 0) {
        return 0;
    }

    let diff = (100 *
        Math.abs(a - b) / ((a + b) / 2));

    return roundToThree(diff);
}

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

export function sortTable(table, sortColumnNumber) {
  var rows, switching, i, x, y, shouldSwitch;
 
  switching = true;

  while (switching) {
    switching = false;
    rows = table.rows;

    for (i = 1; i < (rows.length - 1); i++) {
      shouldSwitch = false;

      x = rows[i].getElementsByTagName("TD")[sortColumnNumber];
      y = rows[i + 1].getElementsByTagName("TD")[sortColumnNumber];

      if (y.innerHTML > x.innerHTML) {
        shouldSwitch = true;
        break;
      }
    }

    if (shouldSwitch) {
      rows[i].parentNode.insertBefore(rows[i + 1], rows[i]);

      switching = true;
    }
  }
}

