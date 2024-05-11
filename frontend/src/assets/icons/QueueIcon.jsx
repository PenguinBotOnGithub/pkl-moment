import React from 'react';

export const QueueIcon = ({size, stroke}) => {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" width={size} height={size} fill="none" viewBox="0 0 192 192">
  <path stroke={stroke} stroke-linecap="round" stroke-linejoin="round" stroke-width="12" d="M30 114v50a6 6 0 0 0 6 6h120a6 6 0 0 0 6-6v-50"/>
  <path stroke={stroke} stroke-linecap="round" stroke-width="12" d="M63 22v115M96 22v115m33-115v115"/>
  <path stroke={stroke} stroke-linecap="round" stroke-linejoin="round" stroke-width="12" d="M22 84V61l16 11.5L22 84Zm132 0V61l16 11.5L154 84Z"/>
</svg>

  );
};