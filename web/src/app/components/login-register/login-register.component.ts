import { Component, OnInit } from '@angular/core';
import { faTimesCircle, faChevronCircleRight, faChevronCircleDown, faEnvelopeSquare } from '@fortawesome/free-solid-svg-icons';
import { FormGroup, FormBuilder, Validators } from '@angular/forms';
import { MatDialogRef } from '@angular/material/dialog';
import { ValidadoresCustomService } from '../../services/validadores-custom.service';
import { Router } from '@angular/router';

@Component({
  selector: 'app-login-register',
  templateUrl: './login-register.component.html',
  styleUrls: ['./login-register.component.scss']
})
export class LoginRegisterComponent implements OnInit {

  // Font Awseome icons
  public faClose = faTimesCircle;
  public faSubmit = faChevronCircleRight;
  public faEmail = faEnvelopeSquare;
  public faSignUp = faChevronCircleDown;

  // Forms
  formLogin: FormGroup;
  formSignUp: FormGroup;

  // Otros atributos
  isLoginOptionShow = true;

  constructor(  private dialogRef: MatDialogRef<LoginRegisterComponent>,
                private fb: FormBuilder,
                private validadoresCustom: ValidadoresCustomService,
                private router: Router ) {

    dialogRef.disableClose = true;

  }

  ngOnInit(): void {

    this.createForms();

  }

  public closeMatDialog(): void {

    this.dialogRef.close();

  }

  private createForms(): void {

    // Login
    this.formLogin = this.fb.group({

      correoUsuario    : ['', [ Validators.required, Validators.minLength(5) ] ],
      password         : ['', [ Validators.required, Validators.minLength(5) ] ],

    });

    // SignUp
    this.formSignUp = this.fb.group({

      correo     : ['', [ Validators.required, Validators.pattern( '[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,3}$' )] ],
      usuario    : ['', [ Validators.required, Validators.minLength(5) ]],
      password1  : ['', [ Validators.required, Validators.minLength(5) ] ],
      password2  : ['', [ Validators.required, Validators.minLength(5) ] ],

    }, {
      // A nivel formulario asyncValidators (sino uno creado)
      validators: this.validadoresCustom.passwordsIguales('password1', 'password2') // Validador personal asyncrono
    });

  }

  public loginUser(): void {

    // Marcar datos inválidos
    if ( this.formLogin.invalid ) {

      return Object.values( this.formLogin.controls ).forEach( control => {

        // Si es un objeto
        if ( control instanceof FormGroup ) {

          Object.values( control.controls ).forEach( subControl => subControl.markAsTouched() );

        } else {

          control.markAsTouched(); // Marcar todos como tocadas

        }

      } );

    } else {

      console.log('TEST > Usuario login correcto');

      localStorage.setItem('tokenJWT', 'adminToken'); // Uso temporal por ahora
      this.router.navigate(['/home']);

      this.closeMatDialog();

    }

  }

  public signUpUser(): void {

    // Marcar datos inválidos
    if ( this.formSignUp.invalid ) {

      return Object.values( this.formSignUp.controls ).forEach( control => {

        // Si es un objeto
        if ( control instanceof FormGroup ) {

          Object.values( control.controls ).forEach( subControl => subControl.markAsTouched() );

        } else {

          control.markAsTouched(); // Marcar todos como tocadas

        }

      } );

    } else {

      console.log('TEST > Usuario Registrado');

      localStorage.setItem('tokenJWT', 'adminToken'); // Uso temporal por ahora
      this.router.navigate(['/home']);

      this.closeMatDialog();

    }


  }

  public changeLoginShow( indexTab: number ): void {

    this.isLoginOptionShow = (indexTab === 1) ? false : true;

  }

  public submitForm(): void {

    if ( this.isLoginOptionShow ) {

      this.loginUser();

    } else {

      this.signUpUser();

    }


  }

  // getters
  get correoUsuarioLoginNovalido(): boolean {
    return ( this.formLogin.get('correoUsuario').invalid && this.formLogin.get('correoUsuario').touched );
  }

  get passwordLoginNoValido(): boolean {
    return ( this.formLogin.get('password').invalid && this.formLogin.get('password').touched );
  }

  get correoSignUpNovalido(): boolean {
    return ( this.formSignUp.get('correo').invalid && this.formSignUp.get('correo').touched );
  }

  get usuarioNoValido(): boolean {
    return ( this.formSignUp.get('usuario').invalid && this.formSignUp.get('usuario').touched );
  }

  get password1SignUpNoValido(): boolean {
    return ( this.formSignUp.get('password1').invalid && this.formSignUp.get('password1').touched );
  }

  get password2SignUpNoValido(): boolean {

    const pass1 = this.formSignUp.get('password1').value;
    const pass2 = this.formSignUp.get('password2').value;

    return ( pass1 === pass2 ) ? false : true;

  }

}