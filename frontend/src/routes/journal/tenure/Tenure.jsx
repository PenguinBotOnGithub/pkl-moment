import React, { useEffect, useState } from "react";
import Cookies from "universal-cookie";
import Search from "../../../components/Search";
import { useNavigate, useParams } from "react-router-dom";
import { fetchDataWrapper } from "../../../services";
import Dropdown from "../../../components/Dropdown";
import { fetchTenure, updateTenure } from "../../../services/functions/tenure";

function Tenure({role}) {
  const [data, setData] = useState([]);
  const [advisorData, setAdvisorData] = useState([]);
  const [dudiData, setDudiData] = useState([]);
  const navigate = useNavigate();
  const [pageData, setPageData] = useState();
  const { page } = useParams();

  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  useEffect(() => {
    fetchDataWrapper(`/api/user`, setAdvisorData, (items) =>
      items.filter((user) => user.role === "advisor_school")
    );
    fetchDataWrapper(`/api/user`, setDudiData, (items) =>
      items.filter((user) => user.role === "advisor_dudi")
    );
  }, []);

  const fetchDataForTenure = async () => {
    try {
      const response = await fetchTenure(page, 10);
      if (response.status != "success") {
        throw new Error(`HTTP error: Status ${response.status}`);
      }
      setPageData(response.data);
      setData(response.data.items);
      setError(null);
    } catch (err) {
      setError(err.message);
      setData([]);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchDataForTenure();
    console.log(data);
  }, [page]);

  function handlePageChange(index) {
    navigate(`/admin/entries/student/${index}`);
  }

  return (
    <>
      <Search />
      <div className="overflow-x-auto">
        <table className="table bg-base-100 border-0 overflow-hidden rounded-lg">
          <thead className="bg-base-300">
            <tr className="border-0">
              <th className="pl-3 pb-2 pr-0 w-0">No</th>
              <th>Nama Siswa</th>
              <th>Pembimbing Sekolah</th>
              <th>Pembimbing Perusahaan</th>
            </tr>
          </thead>
          <tbody className="box-content">
            {data.map((row, index) => (
              <tr key={row.id} className="border-t-2 border-base-300">
                <td className="p-3 pb-2">{index + 1}</td>
                <td>
                  <span>{row.student}</span>
                </td>
                <td>
                  <span>
                    {role == "secretary" || role == "coordinator" ? <Dropdown
                      size="sm"
                      items={advisorData}
                      displayFields={["username"]}
                      searchField={"username"}
                      setSelectedValue={(selectedValue) =>
                        updateTenure(row.id, {
                          advisor_type: "school",
                          advisor_id: selectedValue,
                        })
                      }
                      defaultValue={row.advisor_sch}
                    />:<span>{row.advisor_sch}</span>}
                  </span>
                </td>
                <td>
                  <span>
                    {role == "secretary" || role == "coordinator" ? <Dropdown
                      size="sm"
                      items={dudiData}
                      displayFields={["username"]}
                      searchField={"username"}
                      setSelectedValue={(selectedValue) =>
                        updateTenure(row.id, {
                          advisor_type: "dudi",
                          advisor_id: selectedValue,
                        })
                      }
                      defaultValue={row.advisor_dudi}
                    />:<span>{row.advisor_dudi}</span>}
                  </span>
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
                    pageData.page === index
                      ? "bg-primary text-primary-content"
                      : "bg-base-100"
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
      <button
        className="btn"
        onClick={() => {
          console.log(advisorData);
        }}
      ></button>
    </>
  );
}

export default Tenure;
