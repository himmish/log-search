import Directory from './components/Directory'
import { Navbar } from './components/Navbar'
import Search from './components/Searchbar'

function Home() {

  return (
    <main className="flex flex-col items-center justify-between p-2">
      <Navbar />
      <div className="container grid grid-cols-12 px-8">
        <div className="col-span-3 p-4 container">
          <Directory />
        </div>
        <div className="col-span-8 container">
          <Search />
        </div>

        <div className="col-span-1">
          {"HeatMap"}
        </div>
      </div>
      
    </main>
  )
}

export default Home;