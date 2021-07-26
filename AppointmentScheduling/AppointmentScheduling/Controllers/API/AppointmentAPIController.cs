using AppointmentScheduling.Services;
using Microsoft.AspNetCore.Http;
using Microsoft.AspNetCore.Mvc;
using System.Security.Claims;

namespace AppointmentScheduling.Controllers
{
    [Route("API/Appointment")]
    [ApiController]
    public class AppointmentAPIController : Controller
    {
        private readonly IAppointmentService _appointment_service;
        private readonly IHttpContextAccessor _http_context_accessor;
        private readonly string login_user_id;
        private readonly string role;

        public AppointmentAPIController(IAppointmentService appointment_service, 
                                        IHttpContextAccessor http_context_accessor)
        {
            _appointment_service = appointment_service;
            _http_context_accessor = http_context_accessor;
            login_user_id = _http_context_accessor.HttpContext.User.FindFirstValue(ClaimTypes.NameIdentifier);
            role = _http_context_accessor.HttpContext.User.FindFirstValue(ClaimTypes.Role);
        }

        [HttpPost]
        [Route("SaveCalendarData")]
        public IActionResult SaveCalendarData()
        {
            return View();
        }
    }
}
