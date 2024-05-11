import React from 'react';

export const BurgerIcon = ({size, stroke}) => {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" width={size} height={size} fill="none" viewBox="0 0 192 192">
  <path stroke={stroke} stroke-linecap="round" stroke-linejoin="round" stroke-width="14" d="M23 23h146M23 169h146M23 96h146"/>
</svg>

  );
};