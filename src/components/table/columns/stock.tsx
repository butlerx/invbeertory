import React from 'react';
import { Beer } from '../../../types';
import { right } from '../style.module.scss';

export const stock = ({ row }: { row: Beer }) => <div className={right}>{row.stock}</div>;
