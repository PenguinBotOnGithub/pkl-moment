import React from 'react'

function StUser() {
  return (
    <div className="flex justify-center gap-2">
      <div className="bg-base-100 p-4 rounded-lg flex flex-col items-center flex-1">
        <div className="text-center">
          <span>Total Pengguna</span>
        </div>
        <div className="text-center">
          <span className="text-4xl font-bold">17</span>
        </div>
      </div>
      <div className="bg-base-100 p-4 rounded-lg flex flex-col items-center flex-1">
        <div className="text-center">
          <span>Total Pengguna Yang Pernah Entri</span>
        </div>
        <div className="text-center">
          <span className="text-4xl font-bold">12</span>
        </div>
      </div>
    </div>
  )
}

export default StUser;
