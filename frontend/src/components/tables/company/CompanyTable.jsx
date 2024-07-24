import React, { useEffect, useState } from "react";
import Cookies from "universal-cookie";
import host from "../../../assets/strings/host";

function CompanyTable() {
  const cookies = new Cookies(null, { path: "/" });
  const token = cookies.get("access-token");
  const [data, setData] = useState([]);
  const [isDataEdited, setIsDataEdited] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  const fetchDataForCompanies = async () => {
    try {
      const response = await fetch(`${host}/api/company?page=0`, {
        headers: {
          Authorization: token,
        },
      });
      if (!response.ok) {
        throw new Error(`HTTP error: Status ${response.status}`);
      }
      let companiesData = await response.json();
      setData(companiesData.data.items);
      setIsDataEdited(companiesData.data.items.map(() => false));
      setError(null);
    } catch (err) {
      setError(err.message);
      setData([]);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchDataForCompanies();
  }, []);

  const deleteCompany = async (index) => {
    try {
      const response = await fetch(`${host}/api/company/${index}/delete`, {
        headers: {
          Authorization: token,
        },
        method: "DELETE",
      });
      if (!response.ok) {
        throw new Error(`HTTP error: Status ${response.status}`);
      }
      await response.json();
      setError(null);
      fetchDataForCompanies();
    } catch (err) {
      setError(err.message);
    }
  };

  const handleInputChange = (index, field, value) => {
    const newData = [...data];
    newData[index][field] = value;
    setData(newData);

    const newIsDataEdited = [...isDataEdited];
    newIsDataEdited[index] = true;
    setIsDataEdited(newIsDataEdited);
  };

  const saveChanges = async (index, id) => {
    // Implement the logic to save the changes to the server
    console.log("Form data submitted:", data[index]);
    await fetch(`${host}/api/company/${id}/update`, {
      method: "PATCH",
      headers: {
        "Content-Type": "application/json",
        Authorization: token,
      },
      body: JSON.stringify({
        name: data[index].name,
        address: data[index].address,
      }),
    })
      .then((response) => response.json())
      .then((result) => {
        if (result.status === "success") {
          fetchDataForCompanies();
        }
      })
      .catch(() => {
        alert("Something went wrong");
        fetchDataForCompanies();
      });

    const newIsDataEdited = [...isDataEdited];
    newIsDataEdited[index] = false;
    setIsDataEdited(newIsDataEdited);
  };

  const cancelChanges = () => {
    fetchDataForCompanies();
  };

  return (
    <div className="overflow-x-auto">
      <table className="table bg-base-100 border-0 overflow-hidden rounded-lg">
        <thead className="bg-base-300">
          <tr className="border-0">
            <th className="w-0">No</th>
            <th>Nama Perusahaan</th>
            <th>Alamat</th>
            <th>Aksi</th>
          </tr>
        </thead>
        <tbody className="box-content">
          {data.map((row, index) => (
            <tr key={row.id} className="border-t-2 border-base-300">
              <td>{index + 1}</td>
              <td>
                <input
                  type="text"
                  value={row.name}
                  className="w-full"
                  style={{
                    backgroundColor: "transparent",
                    border: "none",
                    outline: "none",
                  }}
                  onChange={(e) =>
                    handleInputChange(index, "name", e.target.value)
                  }
                />
              </td>
              <td>
                <input
                  type="text"
                  value={row.address}
                  className="w-full"
                  style={{
                    backgroundColor: "transparent",
                    border: "none",
                    outline: "none",
                  }}
                  onChange={(e) =>
                    handleInputChange(index, "address", e.target.value)
                  }
                />
              </td>
              <td>
                {isDataEdited[index] && (
                  <>
                    <button
                      className="btn btn-success btn-xs rounded-lg mr-2"
                      onClick={() => saveChanges(index, row.id)}
                    >
                      Save
                    </button>
                    <button
                      className="btn btn-warning btn-xs rounded-lg mr-2"
                      onClick={() => cancelChanges()}
                    >
                      Cancel
                    </button>
                  </>
                )}
                <button
                  className="btn btn-error btn-xs rounded-lg"
                  onClick={() => deleteCompany(row.id)}
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
