import React from 'react';

export const BooksIcon = ({size, stroke}) => {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" width={size} height={size} fill="none" viewBox="0 0 192 192">
  <rect width={size} height={size} x="22" y="34" stroke={stroke} stroke-linejoin="round" stroke-width="12" rx="6"/>
  <rect width={size} height={size} x="62" y="34" stroke={stroke} stroke-linejoin="round" stroke-width="12" rx="6"/>
  <rect width={size} height={size} x="102.45" y="40.68" stroke={stroke} stroke-linejoin="round" stroke-width="12" rx="6" transform="rotate(-14 102.45 40.68)"/>
</svg>

  );
};