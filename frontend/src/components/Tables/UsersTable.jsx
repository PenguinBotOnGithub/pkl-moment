import React, { useEffect, useState } from "react";
import host from "../../assets/strings/host";
import Cookies from "universal-cookie";

function UsersTable() {
  const [data, setData] = useState([]);
  const cookies = new Cookies(null, { path: "/" });
  const token = cookies.get("access-token");

  const fetchDataForCompanies = async () => {
    try {
      const response = await fetch(`${host}/api/user`, {
        headers: {
          Authorization: token,
        },
      });
      if (!response.ok) {
        throw new Error(`HTTP error: Status ${response.status}`);
      }
      let companiesData = await response.json();
      console.log(companiesData);
      setData(companiesData.data.items);
    } catch (err) {
      alert("something went wrong:"+err);
      setData([]);
    } finally {
    }
  };

  const onDelete = async (index) => {
    try {
      const response = await fetch(`${host}/api/user/${index}/delete`, {
        method: "DELETE",
        headers: {
          Authorization: token,
        },
      });
      if (!response.ok) {
        throw new Error(`HTTP error: Status ${response.status}`);
      }
      fetchDataForCompanies();
    } catch (err) {
      alert("something went wrong:"+err);
    } finally {
    }
  }

  useEffect(() => {
    fetchDataForCompanies();
  },[]);

  return (
    <div className="overflow-x-auto">
      <table className="table bg-base-100 border-0 overflow-hidden rounded-lg">
        <thead className="bg-neutral">
          <tr>
            <th className="w-0">No</th>
            <th>Username</th>
            <th>Role</th>
            <th>Aksi</th>
          </tr>
        </thead>
        <tbody>
          {data.map((row, index) => (
            <tr key={row.id} className="border-t-2 border-neutral">
              <td>{index}</td>
              <td>{row.username}</td>
              <td>{row.role}</td>
              <td>
                <button className="btn btn-warning btn-xs rounded-lg mr-2">
                  Ganti Password
                </button>
                {row.role != "admin" && <button className="btn btn-error btn-xs rounded-lg" onClick={() => {onDelete(row.id)}}>
                  Delete
                </button>}
              </td>
            </tr>
          ))}
        </tbody>
      </table>
      <div className="flex justify-center items-center gap-2 mt-4">
        <button className="flex-none btn bg-base-100">
          <span className="material-symbols-rounded icon-size-20">
            arrow_back
          </span>
        </button>
        <div className="join flex gap-2">
          <button className="join-item btn">1</button>
          <button className="join-item btn">2</button>
          <button className="join-item btn btn-disabled">...</button>
          <button className="join-item btn">99</button>
          <button className="join-item btn">100</button>
        </div>
        <button className="flex-none btn bg-base-100">
          <span className="material-symbols-rounded icon-size-20">
            arrow_forward
          </span>
        </button>
      </div>
      <button onClick={() => {console.log(data)}}>debug button</button>
    </div>
  );
}

export default UsersTable;
