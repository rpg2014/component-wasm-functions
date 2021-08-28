/* tslint:disable */
/* eslint-disable */
/**
* @param {string} input
* @param {any} list
* @param {number} algorithm
* @returns {any}
*/
export function predictive_input(input: string, list: any, algorithm: number): any;
/**
*/
export enum AlgorithmTypes {
  JaroWinkler,
  DatFastShit,
}
/**
*/
export class Prediction {
  free(): void;
/**
* @returns {number}
*/
  score: number;
}
/**
*/
export class PredictiveResults {
  free(): void;
}
