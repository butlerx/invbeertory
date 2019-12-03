import { Beer } from '../../types';

export interface TableBeer extends Beer {
  _original: Beer;
}
