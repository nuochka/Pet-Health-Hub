class Button {
    constructor() {
        this.username = false;
        this.email = false;
        this.password = false;
        this.confirmPassword = false;
    }

    check_valitidy = () => {
        return this.username && this.email && this.password && this.confirmPassword
    }
}

const button = new Button();
const elements = {
    firstname: document.getElementById('firstname'),
    lastname: document.getElementById('lastname'),
    email: document.getElementById('email'),
    password: document.getElementById('password'),
    confirmPassword: document.getElementById('confirmPassword'),
    submitBtn: document.getElementById('submitBtn')
};

const errors = {
    firstname: document.getElementById('firstname_error'),
    lastname: document.getElementById('lastname_error'),
    email: document.getElementById('email_error'),
    password: document.getElementById('password_error'),
    confirmPassword: document.getElementById('confirmPassword_error'),
    submitBtn: document.getElementById('submitBtn_error')
}

const validateField = (field, isValid) => {
    field.classList.toggle('is-valid', isValid);
};

const validateName = () => {
    const field = elements.firstname;
    const nameRegex = /^[a-zA-Z]+$/;
    const isValid = nameRegex.test(field.value);
    validateField(field, isValid);
    button.username = isValid;
    errors.firstname.style.display = !isValid ? "block" : "none";
};

const validateSurname = () => {
    const field = elements.lastname;
    const nameRegex = /^[a-zA-Z]+$/;
    const isValid = nameRegex.test(field.value);
    validateField(field, isValid);
    errors.lastname.style.display = !isValid ? "block" : "none";
};

const validatePasswordMatch = () => {
    const { password, confirmPassword, passwordError } = elements;
    const passwordRegex = /^(?=.*\d)(?=.*[A-Z]).{8,}$/;
    const isBothFieldsFilled = password.value !== '' && confirmPassword.value !== '' && passwordRegex.test(password.value);
    const isValid = isBothFieldsFilled && password.value === confirmPassword.value;

    validateField(password, isValid);
    validateField(confirmPassword, isValid);
    button.password = isValid;
    button.confirmPassword = isValid;
    errors.password.style.display = !isValid ? "block" : "none";
    errors.confirmPassword.style.display = !isValid ? "block" : "none";
    checkFormValidity();
};

const validateEmailFormat = () => {
    const { email, emailFormatError } = elements;
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    const isValid = email.value === '' || emailRegex.test(email.value);
    validateField(email, isValid);
    button.email = isValid;
    errors.email.style.display = !isValid ? "block" : "none";
    checkFormValidity();
};

const checkFormValidity = () => {
    elements.submitBtn.disabled = !button.check_valitidy();
};

elements.email.addEventListener('input', validateEmailFormat);
elements.password.addEventListener('input', validatePasswordMatch);
elements.confirmPassword.addEventListener('input', validatePasswordMatch);
elements.firstname.addEventListener('input', validateName);
elements.lastname.addEventListener('input', validateSurname);
elements.email.addEventListener('input', validateEmailFormat);
elements.password.addEventListener('input', validatePasswordMatch);
elements.confirmPassword.addEventListener('input', validatePasswordMatch);
