using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using InAndOut.Models;
using Microsoft.EntityFrameworkCore;

namespace InAndOut.Data
{
    public class ApplicationDBContext : DbContext
    {
        public ApplicationDBContext(DbContextOptions<ApplicationDBContext> options) : base(options)
        {
            
        }

        public DbSet<Item> Items { get; set; }
        public DbSet<Expense> Expenses { get; set; }
    }
}
