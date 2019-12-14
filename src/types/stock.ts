import { Beer } from './beer';
import { Coffee } from './coffee';

export interface StockProps {
  data: {
    allGoogleInventorySheet: {
      nodes: Beer[];
    };
  };
}
