﻿using Microsoft.AspNetCore.Mvc.Rendering;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace AppointmentScheduling.Utility
{
    public class Helper
    {
        private static string admin = "Admin";
        private static string patient = "Patient";
        private static string doctor = "Doctor";

        public static string Admin { get => admin; set => admin = value; }
        public static string Patient { get => patient; set => patient = value; }
        public static string Doctor { get => doctor; set => doctor = value; }

        public static List<SelectListItem> GetRolesForDropDown()
        {
            return new List<SelectListItem>
            {
                new SelectListItem{Value = Helper.Admin, Text = Helper.Admin},
                new SelectListItem{Value = Helper.Patient, Text = Helper.Patient},
                new SelectListItem{Value = Helper.Doctor, Text = Helper.Doctor},
            };
        }
    }
}
