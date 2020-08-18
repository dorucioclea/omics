import { HttpEvent, HttpHandler, HttpInterceptor, HttpRequest } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';

@Injectable()
export class AuthService {
  private authToken: string;

  constructor() {
    const authToken = localStorage.getItem('auth_token');
    if (authToken) {
      this.authToken = authToken;
    }
  }

  public setToken(authToken: string): void {
    this.authToken = authToken;
  }

  public getToken(): string {
    return this.authToken;
  }

  public isLoggedIn(): boolean {
    return !!this.authToken;
  }

}

@Injectable()
export class AuthInterceptor implements HttpInterceptor {
  constructor(private authServ: AuthService) {  }

  public intercept(req: HttpRequest<any>, next: HttpHandler): Observable<HttpEvent<any>> {
    req = req.clone({
      setHeaders: {
        'Accept': 'application/json',
        'Authorization': `Bearer ${this.authServ.getToken()}`,
        'Content-Type': 'application/json; charset=utf-8',
      },
    });

    return next.handle(req);
  }
}