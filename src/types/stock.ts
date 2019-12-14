import { Beer } from './beer';
import { Coffee } from './coffee';

export interface StockProps {
  data: {
    googleSheet: {
      inventory: Beer[];
    };
  };
}
