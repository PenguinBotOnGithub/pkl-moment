import React from 'react';

export const SunIcon = ({size, stroke}) => {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" width={size} height={size} fill="none" viewBox="0 0 192 192">
  <path stroke={stroke} stroke-width="12" d="M124.991 124.991C117.572 132.411 107.322 137 96 137s-21.572-4.589-28.991-12.009C59.589 117.572 55 107.322 55 96s4.59-21.572 12.009-28.991C74.429 59.589 84.679 55 96 55c11.322 0 21.572 4.59 28.991 12.009C132.411 74.429 137 84.679 137 96c0 11.322-4.589 21.572-12.009 28.991Z"/>
  <path stroke={stroke} stroke-linecap="round" stroke-linejoin="round" stroke-width="12" d="M156 96h14M96 36V22M36 96H22m74 60v14m42.426-31.574 9.9 9.9m-94.752-9.9-9.9 9.9m9.9-94.752-9.9-9.9m94.752 9.9 9.9-9.9"/>
</svg>

  );
};