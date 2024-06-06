import React, { useEffect, useState } from "react";
import Cookies from "universal-cookie";
import host from "../../assets/strings/host";

function CompanyTable() {
  const cookies = new Cookies(null, { path: "/" });
  const token = cookies.get("access-token");
  const [data, setData] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  useEffect(() => {
    const fetchDataForCompanies = async () => {
      try {
        const response = await fetch(`${host}/api/company?page=0`, {
          headers: {
            Authorization: token,
          },
        });
        if (!response.status) {
          throw new Error(`HTTP error: Status ${response.status}`);
        }
        let companiesData = await response.json();
        setData(companiesData.data.items);
        setError(null);
      } catch (err) {
        setError(err.message);
        setData(null);
      } finally {
        setLoading(false);
      }
    };

    fetchDataForCompanies();
  });

  const deleteCompany = async (index) => {
    console.log("dfasdfasf");
    try {
      const response = await fetch(`${host}/api/company/${index}/delete`, {
        headers: {
          Authorization: token,
        },
        method: "DELETE",
      });
      if (!response.status) {
        throw new Error(`HTTP error: Status ${response.status}`);
      }
      let companiesData = await response.json();
      setError(null);
    } catch (err) {
      setError(err.message);
      setData(null);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="overflow-x-auto">
      <table className="table bg-base-100 border-0 overflow-hidden rounded-lg">
        <thead className="bg-neutral">
          <tr className="border-0">
            <th className="w-0">No</th>
            <th>Nama Perusahaan</th>
            <th>Alamat</th>
            <th>Aksi</th>
          </tr>
        </thead>
        <tbody className="box-content">
          {data.map((row, index) => (
            <tr key={row.id} className="border-t-2 border-neutral">
              <td>{index + 1}</td>
              <td>{row.name}</td>
              <td>{row.address}</td>
              <td>
                <button className="btn btn-info btn-xs rounded-lg mr-2">
                  Edit
                </button>
                <button
                  className="btn btn-error btn-xs rounded-lg"
                  onClick={deleteCompany(row.id)}
                >
                  Delete
                </button>
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
          <button className="join-item btn opacity-50">...</button>
          <button className="join-item btn">99</button>
          <button className="join-item btn">100</button>
        </div>
        <button className="flex-none btn bg-base-100">
          <span className="material-symbols-rounded icon-size-20">
            arrow_forward
          </span>
        </button>
      </div>
    </div>
  );
}

export default CompanyTable;
