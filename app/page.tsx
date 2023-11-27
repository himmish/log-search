import Directory from './components/Directory'
import { Navbar } from './components/Navbar'
import Search from './components/Searchbar'
import Viewer from './components/viewer/Viewer';

function Home() {

  return (
    <main className="h-max flex flex-col items-center justify-between p-2">
      <Navbar />
      <div className="container grid grid-cols-12 px-8">
        <div className="col-span-3 py-2 container h-screen overflow-y-auto">
          <Directory />
        </div>
        <div className="col-span-9 container overflow-hidden">
          <Search />
          <Viewer />
        </div>
      </div>
    </main>
  )
}

export default Home;