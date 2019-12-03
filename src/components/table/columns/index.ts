import { size } from './size';
import { year } from './year';
import { abv } from './abv';
import { stock } from './stock';
import { style } from './style';
import { brewery } from './brewery';

export const columns = [
  { Header: 'Name', accessor: 'name', minWidth: 50 },
  { Header: 'Brewery', accessor: 'brewery', minWidth: 26, Cell: brewery },
  {
    Header: 'Year',
    accessor: 'year',
    minWidth: 6,
    Cell: year,
  },
  {
    Header: 'ABV',
    accessor: 'abv',
    minWidth: 6,
    Cell: abv,
  },
  { Header: 'Style', accessor: 'style', minWidth: 40, Cell: style },
  {
    Header: 'Size',
    accessor: 'size',
    minWidth: 8,
    Cell: size,
  },
  {
    Header: 'Stock',
    accessor: 'stock',
    minWidth: 6,
    Cell: stock,
  },
];
