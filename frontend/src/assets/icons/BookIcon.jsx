import React from 'react';

export const BookIcon = ({size, stroke}) => {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" width={size} height={size} fill="none" viewBox="0 0 192 192">
  <rect width={size} height={size} x="46" y="22" stroke={stroke} stroke-linejoin="round" stroke-width="12" rx="6"/>
  <path stroke={stroke} stroke-linecap="round" stroke-linejoin="round" stroke-width="12" d="M124 28v44l-14-11.294L96 72V28a6 6 0 0 1 6-6h16a6 6 0 0 1 6 6Z"/>
</svg>

  );
};