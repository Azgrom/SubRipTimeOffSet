using Microsoft.AspNetCore.Mvc.Rendering;
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
        private static string appointment_added = "Appointment added successfully.";
        private static string appointment_updated = "Appointment updated successfully.";
        private static string appointment_deleted = "Appointment deleted successfully.";
        private static string appointment_exists = "Appointment for selected date and time already exists.";
        private static string appointment_not_exists = "Appointment not exists.";

        private static string something_went_wrong = "Sometring went wrong, please try again.";
        private static int success_code = 1;
        private static int failure_code = 0;

        public static string Admin { get => admin; set => admin = value; }
        public static string Patient { get => patient; set => patient = value; }
        public static string Doctor { get => doctor; set => doctor = value; }
        public static string Appointment_added { get => appointment_added; set => appointment_added = value; }
        public static string Appointment_updated { get => appointment_updated; set => appointment_updated = value; }
        public static string Appointment_deleted { get => appointment_deleted; set => appointment_deleted = value; }
        public static string Appointment_exists { get => appointment_exists; set => appointment_exists = value; }
        public static string Appointment_not_exists { get => appointment_not_exists; set => appointment_not_exists = value; }
        public static string Appointment_add_error { get => something_went_wrong; set => something_went_wrong = value; }
        public static string Appointment_update_error { get => something_went_wrong; set => something_went_wrong = value; }
        public static string Something_went_wrong { get => something_went_wrong; set => something_went_wrong = value; }
        public static int Success_code { get => success_code; set => success_code = value; }
        public static int Failure_code { get => failure_code; set => failure_code = value; }

        public static List<SelectListItem> GetRolesForDropDown()
        {
            return new List<SelectListItem>
            {
                new SelectListItem{Value = Helper.Admin, Text = Helper.Admin},
                new SelectListItem{Value = Helper.Patient, Text = Helper.Patient},
                new SelectListItem{Value = Helper.Doctor, Text = Helper.Doctor},
            };
        }

        public static List<SelectListItem> GetTimeDropDown()
        { 
            int minute = 60;
            List<SelectListItem> duration = new();

            for (int i = 1; i <= 12; i++)
            {
                duration.Add(new SelectListItem { Value = minute.ToString(), Text = i + " Hr" });
                minute += 30;
                duration.Add(new SelectListItem { Value = minute.ToString(), Text = i + " Hr 30 min" });
                minute += 30;
            }

            return duration;
        }
    }
}
