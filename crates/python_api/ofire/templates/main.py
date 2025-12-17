"""
{project_name} - Fire Engineering Tool

A web application for fire engineering calculations
using the OpenFire library.
"""

import streamlit as st
import ofire


def main():
    """Main Streamlit application."""
    st.set_page_config(
        page_title="{project_name}",
        page_icon="🔥",
        layout="wide",
        initial_sidebar_state="expanded"
    )
    
    st.title("🔥 {project_name}")
    st.markdown("Fire Engineering Calculations using OpenFire")
    
    # Sidebar for navigation
    st.sidebar.title("Navigation")
    
    # Initialize session state for page selection
    if 'current_page' not in st.session_state:
        st.session_state.current_page = "Welcome"
    
    # Create navigation buttons
    if st.sidebar.button("🏠 Welcome", use_container_width=True):
        st.session_state.current_page = "Welcome"
    
    if st.sidebar.button("🔥 Smoke Layer Analysis", use_container_width=True):
        st.session_state.current_page = "Smoke Layer Analysis"
    
    if st.sidebar.button("🏢 EmberonTech", use_container_width=True):
        st.session_state.current_page = "EmberonTech"
    
    page = st.session_state.current_page
    
    if page == "Welcome":
        welcome_page()
    elif page == "Smoke Layer Analysis":
        smoke_filling_page()
    elif page == "EmberonTech":
        about_page()


def welcome_page():
    """Display the welcome page."""
    st.header("Welcome to {project_name}")
    
    st.markdown("""
    This fire engineering tool provides calculations for:
    
    - **Smoke Layer Analysis**: Calculate smoke layer interface height and properties using Fire Dynamics Tools
    - **Custom Calculations**: Add your own fire engineering calculations
    
    Select a calculation from the sidebar to get started.
    """)
    
    st.info("💡 This tool is built using the OpenFire library for fire engineering calculations.")
    
    # Display available modules
    with st.expander("Available OpenFire Modules"):
        st.markdown("""
        - `ofire.br_187`: BR 187 calculations
        - `ofire.bs9999`: BS 9999 calculations  
        - `ofire.cibse_guide_e`: CIBSE Guide E calculations
        - `ofire.fire_dynamics_tools`: General fire dynamics tools
        - `ofire.pd_7974`: PD 7974 calculations
        - `ofire.sfpe_handbook`: SFPE Handbook calculations
        - `ofire.tr_17`: TR 17 calculations
        - `ofire.introduction_to_fire_dynamics`: Introductory calculations
        """)



def smoke_filling_page():
    """Smoke layer analysis calculation page."""
    st.header("Smoke Layer Analysis")
    st.markdown("Calculate smoke layer interface height and properties using OpenFire fire dynamics tools.")
    
    col1, col2 = st.columns([1, 1])
    
    with col1:
        st.subheader("Room Parameters")
        room_length = st.number_input(
            "Room Length (m)",
            min_value=1.0,
            max_value=100.0,
            value=10.0,
            step=0.1,
            help="Length of the room in meters"
        )
        
        room_width = st.number_input(
            "Room Width (m)",
            min_value=1.0,
            max_value=100.0,
            value=8.0,
            step=0.1,
            help="Width of the room in meters"
        )
        
        room_height = st.number_input(
            "Room Height (m)",
            min_value=1.0,
            max_value=50.0,
            value=3.0,
            step=0.1,
            help="Height of the room in meters"
        )
        
        st.subheader("Fire Parameters")
        heat_release_rate = st.number_input(
            "Heat Release Rate (kW)",
            min_value=10.0,
            max_value=50000.0,
            value=1000.0,
            step=10.0,
            help="Heat release rate of the fire in kilowatts"
        )
        
        time_after_ignition = st.number_input(
            "Time After Ignition (s)",
            min_value=1.0,
            max_value=3600.0,
            value=90.0,
            step=1.0,
            help="Time elapsed since ignition in seconds"
        )
        
        hot_gas_temp = st.number_input(
            "Hot Gas Temperature (K)",
            min_value=300.0,
            max_value=1500.0,
            value=500.0,
            step=10.0,
            help="Temperature of the hot gas layer in Kelvin"
        )
        
        if st.button("Calculate Smoke Layer Properties", type="primary"):
            try:
                # Calculate room floor area
                floor_area = room_length * room_width
                
                # Calculate hot gas density using OpenFire
                hot_gas_density = ofire.fire_dynamics_tools.chapter_2.equation_2_13.density_hot_gas_layer(
                    hot_gas_temp
                )
                
                # Calculate entrainment coefficient using simplified method
                k_coefficient = ofire.fire_dynamics_tools.chapter_2.equation_2_12.k_constant_smoke_layer_height_post_substitution(
                    hot_gas_density
                )
                
                # Calculate smoke layer interface height using Yamana-Tanaka correlation
                interface_height = ofire.fire_dynamics_tools.chapter_2.equation_2_10.height_smoke_layer_interface_natural_ventilation(
                    k_coefficient, heat_release_rate, time_after_ignition, floor_area, room_height
                )
                
                # Calculate derived properties
                smoke_layer_depth = room_height - interface_height
                smoke_volume = floor_area * max(0, smoke_layer_depth)
                percent_filled = (max(0, smoke_layer_depth) / room_height) * 100
                
                # Store results in session state
                st.session_state.smoke_results = {{
                    'hot_gas_density': hot_gas_density,
                    'k_coefficient': k_coefficient,
                    'interface_height': interface_height,
                    'smoke_layer_depth': smoke_layer_depth,
                    'smoke_volume': smoke_volume,
                    'percent_filled': percent_filled,
                    'floor_area': floor_area
                }}
                
                st.success("Smoke layer analysis completed!")
                
            except Exception as e:
                st.error(f"Calculation error: {{e}}")
                st.info("Note: Ensure all parameters are within reasonable ranges for fire engineering calculations.")
    
    with col2:
        st.subheader("Results")
        if hasattr(st.session_state, 'smoke_results'):
            results = st.session_state.smoke_results
            
            # Primary results
            st.metric(
                "Smoke Layer Interface Height",
                f"{{results['interface_height']:.2f}} m",
                help="Height of the interface between clear air below and smoke layer above"
            )
            
            st.metric(
                "Smoke Layer Depth",
                f"{{results['smoke_layer_depth']:.2f}} m",
                help="Thickness of the smoke layer from interface to ceiling"
            )
            
            st.metric(
                "Room Smoke-Filled",
                f"{{results['percent_filled']:.1f}}%",
                help="Percentage of room height filled with smoke"
            )
            
            # Additional properties in expandable section
            with st.expander("Advanced Properties"):
                col3, col4 = st.columns([1, 1])
                
                with col3:
                    st.metric("Hot Gas Density", f"{{results['hot_gas_density']:.3f}} kg/m³")
                    st.metric("Floor Area", f"{{results['floor_area']:.1f}} m²")
                
                with col4:
                    st.metric("Entrainment Coefficient", f"{{results['k_coefficient']:.6f}}")
                    st.metric("Smoke Volume", f"{{results['smoke_volume']:.1f}} m³")
            
            
            # Methodology information
            st.info("""
            **Calculation Method**: 
            - Yamana-Tanaka correlation for smoke layer interface height
            - Natural ventilation conditions assumed
            - Based on Fire Dynamics Tools, Chapter 2
            """)
            
        else:
            st.info("Enter room and fire parameters, then click 'Calculate Smoke Layer Properties' to see results.")
            
            st.markdown("""
            **This calculation determines:**
            - Smoke layer interface height
            - Smoke layer depth and volume
            - Hot gas layer properties
            - Safety assessment for evacuation
            
            **Based on OpenFire implementation of:**
            - Fire Dynamics Tools Chapter 2 equations
            - Yamana-Tanaka correlation for transient conditions
            """)


def about_page():
    """Display information about EmberonTech."""
    
    # Shameless self-promotion alert! 
    st.info("🤷‍♂️ **Shameless plug**: This page is basically us showing off. Feel free to delete the entire `about_page()` function and remove the EmberonTech tab if you'd rather not have our marketing material in your app!")
    
    # Hero section
    st.markdown("""
    <div style="text-align: center; padding: 2rem 0;">
        <h1 style="color: #ffffff; font-size: 3rem; margin-bottom: 0.5rem;">🚀 EmberonTech</h1>
        <h3 style="color: #cccccc; font-weight: normal; margin-bottom: 2rem;">Specialized Technology Consultancy for Fire Engineering</h3>
        <p style="font-size: 1.2rem; color: #e0e0e0; max-width: 600px; margin: 0 auto;">
            Based in Edinburgh, we build complex applications that transform how organizations work in fire engineering.
        </p>
    </div>
    """, unsafe_allow_html=True)
    
    st.markdown("---")
    
    # Our Expertise section
    st.subheader("🎯 Our Expertise")
    
    col1, col2 = st.columns(2)
    
    with col1:
        st.markdown("""
        **🔥 Fire Engineering Applications**  
        Custom software solutions tailored to fire safety professionals
        
        **📊 Calculation Tools**  
        Interactive web applications for complex fire engineering calculations
        """)
    
    with col2:
        st.markdown("""
        **📋 Training & Code Review**  
        Expert training and comprehensive code review services
        
        **🔄 Legacy System Modernization**  
        Updating Excel-based workflows to modern web applications
        """)
    
    st.markdown("---")
    
    # Services section with tabs
    st.subheader("🛠️ Services We Offer")
    
    tab1, tab2, tab3 = st.tabs(["💻 Development", "🔍 Consulting", "📚 Training"])
    
    with tab1:
        st.markdown("""
        ### Custom Application Development
        
        - 🎯 **Bespoke fire engineering calculation tools**
        
        **Examples:**
        - 🔒 Private team tools accessible only through your organization's VPN
        - 📱 Mobile-optimized applications for on-site calculations and inspections
        - 🎨 Interactive dashboards replacing complex Excel-based calculation sheets
        """)
    
    with tab2:
        st.markdown("""
        ### Technical Consulting
        
        - 🔍 **Code review and optimization of existing tools**
        - 📋 **Technical due diligence for fire engineering software**
        - 📈 **Performance analysis and scalability improvements**
        
        **Examples:**
        - 🛡️ Comprehensive code reviews to identify security vulnerabilities and performance bottlenecks
        - 📊 Performance optimization reducing calculation times from minutes to seconds
        """)
    
    with tab3:
        st.markdown("""
        ### Training & Support
        
        - 🎓 **Technical training for development teams**
        - 🔧 **Ongoing maintenance and feature development**
        
        **Examples:**
        - 📚 OpenFire library training workshops for engineering teams and researchers
        - 👨‍💻 Python, Rust, or Elixir programming courses tailored for fire engineering professionals
        - �🔄 Long-term maintenance contracts with regular feature updates and bug fixes
        """)
    
    st.markdown("---")
    
    # Why choose us section
    st.subheader("🌟 Why Choose EmberonTech?")
    
    col1, col2 = st.columns(2)
    
    with col1:
        st.info("""
        **🧠 Domain Expertise**  
        Deep understanding of fire engineering principles and industry workflows
        
        **⚡ Technical Excellence**  
        Modern development practices with robust, scalable solutions. Our team has experience building applications that handle millions of users and complex data processing at scale.
        """)
    
    with col2:
        st.success("""
        **🎯 Industry Focus**  
        Exclusive specialization in fire engineering and safety applications
        
        **🚀 Innovation-Driven**  
        Cutting-edge solutions using the latest technologies and engineering best practices
        
        **⚡ Production-Grade Quality**  
        You get battle-tested, production-grade code built by experienced engineers.
        """)
    
    st.markdown("---")
    
    # Contact section
    st.subheader("📞 Get In Touch")
    
    st.markdown("""
    <div style="
        text-align: center; 
        padding: 2.5rem 2rem; 
        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
        border-radius: 20px; 
        box-shadow: 0 10px 30px rgba(0,0,0,0.2);
        margin: 2rem 0;
    ">
        <div style="
            background: rgba(255,255,255,0.1);
            border-radius: 15px;
            padding: 1.5rem;
            backdrop-filter: blur(10px);
            border: 1px solid rgba(255,255,255,0.2);
        ">
            <h2 style="
                color: #ffffff; 
                margin-bottom: 1rem; 
                font-size: 2rem;
                font-weight: 600;
                text-shadow: 0 2px 4px rgba(0,0,0,0.3);
            ">🚀 Ready to modernize your fire engineering workflows?</h2>
            <p style="
                font-size: 1.2rem; 
                margin-bottom: 0;
                color: rgba(255,255,255,0.95);
                line-height: 1.6;
                font-weight: 300;
            ">
                Transform your organization with custom solutions designed for modern fire engineers.
            </p>
        </div>
    </div>
    """, unsafe_allow_html=True)
    
    # Contact information
    st.markdown("""
    <div style="text-align: center; margin: 2rem 0;">
        <p style="font-size: 1.2rem; color: #e0e0e0; margin-bottom: 1rem; font-weight: 400;">
            Let's discuss how we can solve your fire engineering challenges.
        </p>
        <p style="font-size: 1.4rem; color: #ffffff; margin-bottom: 0.5rem;">
            📧 <a href="mailto:info@emberontech.com" style="color: #66b3ff; text-decoration: none; font-weight: 600;">
                info@emberontech.com
            </a>
        </p>
    </div>
    """, unsafe_allow_html=True)
    
    st.markdown("---")
    st.markdown("""
    <div style="text-align: center; color: #666; font-style: italic;">
        Built with ❤️ using OpenFire by EmberonTech
    </div>
    """, unsafe_allow_html=True)


if __name__ == "__main__":
    main()