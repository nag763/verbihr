import init from './verbihr.js';
init().catch((e) => {
    document.getElementById('status').remove();
    console.log("Error while creating project");
});