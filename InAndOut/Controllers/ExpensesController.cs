using InAndOut.Data;
using InAndOut.Models;
using Microsoft.AspNetCore.Mvc;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace InAndOut.Controllers
{
    public class ExpensesController : Controller
    {
        private readonly ApplicationDBContext _db;

        public ExpensesController(ApplicationDBContext db)
        {
            _db = db;
        }

        // GET
        public IActionResult Index()
        {
            IEnumerable<Expense> objList = _db.Expenses;
            return View(objList);
        }

        // GET
        public IActionResult Create()
        {
            return View();
        }
    }
}
