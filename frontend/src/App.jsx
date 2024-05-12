import { createBrowserRouter, RouterProvider } from "react-router-dom";
import UsageExample from "./components/UsageExample";
import './i18n';

const router = createBrowserRouter([
  {
    path: "/",
    element: <UsageExample />,
  },
]);

function App() {
  return <RouterProvider router={router} />;
}

export default App;
