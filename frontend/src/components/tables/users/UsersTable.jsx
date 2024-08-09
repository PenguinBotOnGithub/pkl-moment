import React, { useEffect, useState } from "react";
import Cookies from "universal-cookie";
import { useNavigate, useParams } from "react-router-dom";
import { fetchData } from "../../../services";

function UsersTable() {
  const navigate = useNavigate();
  const [data, setData] = useState([]);
  const [pageData, setPageData] = useState();
  const { page } = useParams();
  const cookies = new Cookies(null, { path: "/" });
  const token = cookies.get("access-token");

  function handlePageChange(index) {
    navigate(`/admin/users/${index}`);
  }

  const fetchDataForUsers = async () => {
    try {
      const response = await fetchData(`/api/user?page=${page}&size=${cookies.get("max-item-users")}`);
      setData(response.data.items);
      setPageData(response.data);
    } catch (err) {
      alert("something went wrong:" + err);
      setData([]);
    } finally {
    }
  };

  const onDelete = async (index) => {
    try {
      await fetchData(`/api/user/${index}/delete`, {
        method: "DELETE",
      });
      fetchDataForUsers();
    } catch (err) {
      alert("something went wrong:" + err);
    } finally {
    }
  };

  useEffect(() => {
    fetchDataForUsers();
  }, [page]);

  return (
    <div className="overflow-x-auto">
      <table className="table bg-base-100 border-0 overflow-hidden rounded-lg">
        <thead className="bg-base-300">
          <tr>
            <th className="w-0">No</th>
            <th>Username</th>
            <th>Role</th>
            <th>Aksi</th>
          </tr>
        </thead>
        <tbody>
          {data.map((row, index) => (
            <tr key={row.id} className="border-t-2 border-base-300">
              <td>{index + 1}</td>
              <td>{row.username}</td>
              <td>{row.role}</td>
              <td>
                <button className="btn btn-warning btn-xs rounded-lg mr-2">
                  Ganti Password
                </button>
                {row.role != "secretary" && (
                  <button
                    className="btn btn-error btn-xs rounded-lg"
                    onClick={() => {
                      onDelete(row.id);
                    }}
                  >
                    Delete
                  </button>
                )}
              </td>
            </tr>
          ))}
        </tbody>
      </table>
      {pageData && (
          <div className="flex justify-center items-center gap-2 mt-2">
            <button
              className="flex-none btn bg-base-100"
              onClick={() => handlePageChange(pageData.page - 1)}
              disabled={pageData.page === 0}
            >
              <span className="material-symbols-rounded icon-size-20">
                arrow_back
              </span>
            </button>
            <div className="join flex">
              {[...Array(pageData.num_pages)].map((_, index) => (
                <button
                  key={index}
                  className={`join-item btn ${
                    pageData.page === index ? "bg-primary text-primary-content" : "bg-base-100"
                  }`}
                  onClick={() => handlePageChange(index)}
                >
                  {index + 1}
                </button>
              ))}
            </div>
            <button
              className="flex-none btn bg-base-100"
              onClick={() => handlePageChange(pageData.page + 1)}
              disabled={pageData.page === pageData.num_pages - 1}
            >
              <span className="material-symbols-rounded icon-size-20">
                arrow_forward
              </span>
            </button>
          </div>
        )}
    </div>
  );
}

export default UsersTable;
