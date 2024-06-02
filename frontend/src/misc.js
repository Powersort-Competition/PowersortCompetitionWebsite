export function constructSubmissionInputPayload(inputArr, submissionId) {
  inputArr = inputArr.append("submissionId", submissionId);

  return inputArr;
}

export function getInputSize(inputArr) {
  inputArr = inputArr.split(",");

  return inputArr.length;
}
