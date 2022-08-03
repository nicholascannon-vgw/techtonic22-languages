const isEmptyString = (value) => value.trim() === '';

const notString = (value) => typeof value !== 'string';

module.exports = {
    isEmptyString,
    notString,
};
