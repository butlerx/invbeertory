import React from 'react';
import { Beer } from '../../../types';
import { center } from '../style.module.scss';

export const year = ({ row }: { row: Beer }) => <div className={center}>{row.year}</div>;
