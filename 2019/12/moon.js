var stepsize = 100;
function Particle( x,y,z,c )
{
	this._pos = new Vec([x*stepsize,y*stepsize,z*stepsize]);
	this._radius = stepsize;
	this._c =c;

	this._from_v = new Vec();
	this._to_v = new Vec();

	this._from_p = new Vec([x*stepsize,y*stepsize,z*stepsize]);
	this._to_p = new Vec([x*stepsize,y*stepsize,z*stepsize]);
};

function dotick(p1,p2){
	if( p1._to_p.data[0] > p2._to_p.data[0] ){
		p1._to_v.data[0] -= stepsize;
		p2._to_v.data[0] += stepsize;
	} else if( p1._to_p.data[0] < p2._to_p.data[0] ){
		p1._to_v.data[0] += stepsize;
		p2._to_v.data[0] -= stepsize;
	}
	if( p1._to_p.data[1] > p2._to_p.data[1] ){
		p1._to_v.data[1] -= stepsize;
		p2._to_v.data[1] += stepsize;
	} else if( p1._to_p.data[1] < p2._to_p.data[1] ){
		p1._to_v.data[1] += stepsize;
		p2._to_v.data[1] -= stepsize;
	}
	if( p1._to_p.data[2] > p2._to_p.data[2] ){
		p1._to_v.data[2] -= stepsize;
		p2._to_v.data[2] += stepsize;
	} else if( p1._to_p.data[2] < p2._to_p.data[2] ){
		p1._to_v.data[2] += stepsize;
		p2._to_v.data[2] -= stepsize;
	}

}

		Particle.prototype.pos = function() { return this._pos; };
		Particle.prototype.tick = function(dt){
			this._pos = new Vec([
				(1-dt)*this._from_p.data[0]+dt*this._to_p.data[0],
				(1-dt)*this._from_p.data[1]+dt*this._to_p.data[1],
				(1-dt)*this._from_p.data[2]+dt*this._to_p.data[2]
			])
		}
		Particle.prototype.nextstep = function(){
			this._from_v = this._to_v;
			this._from_p = this._to_p;
		}
		Particle.prototype.move = function(dt) {

			this._to_p = this._to_p.add(this._to_v);
		};
		Particle.prototype.r = function(){ return this._radius;};
		Particle.prototype.style = function(){return this._c;};

var setVpMovementActions = function( view )
{
	var movevp = function( vp, direction, multiplier ) {
		vp.moveTo( vp.pos().add( direction.mul( multiplier ) ) )
	};
	var addMoveActions = function( a, s, selector )
	{
		view.addAction( a, function(vp,c){ movevp( vp, selector(vp), stepsize/10 ) } );
		view.addAction( s, function(vp,c){ movevp( vp, selector(vp), stepsize/-10 ) } );
	};
	addMoveActions( 87, 83, function(vp){ return vp.n() } ); // w s
	addMoveActions( 68, 65, function(vp){ return vp.u() } ); // d a
	addMoveActions( 81, 69, function(vp){ return vp.v() } ); // q e
	view.addAction( 88, //x
	function(vp){ vp.reset() } );
	var rotatevp = function( vp, angle, about )
	{
		vp.rotate( angle, about );
	};
	var addRotActions = function( a, s, selector)
	{
		var rotatespeed = 0.005*2*Math.PI;
		view.addAction( a, function(vp){rotatevp( vp, rotatespeed, selector(vp) ) } );
		view.addAction( s, function(vp){rotatevp( vp, -1*rotatespeed, selector(vp) ) } );
	};

	addRotActions( 73,75,function(vp){return vp.u() }); //ik
	addRotActions( 76,74,function(vp){return vp.v() }); //lj
	addRotActions( 79,85,function(vp){return vp.n() }); //ou
};
var makeMoonSim = function()
{
	var _drawables = []
	var vp_start = new Vec([0,0,-2000]);
	var xdir = new Vec([1,0,0]);
	var ydir = new Vec([0,1,0]);;

	var viewport = makeView(vp_start,xdir,ydir);
	setVpMovementActions( viewport );

	_drawables.push(new Particle(-4,3,15,"yellow"));
	_drawables.push(new Particle(-11,-10,13,"blue"));
	_drawables.push(new Particle(2,2,18,"green"));
	_drawables.push(new Particle(7,-1,0,"gray"));


	var mid = new Particle(0,0,0,"white");
	var steptrigger = 0;
	var lastmark = -1;



	return function( context, width, height, mark, keys ) {
		var step_time = 100;
		if(mark > steptrigger ){
			steptrigger = mark + step_time;
			lastmark = mark;
			for( var d = 0; d<_drawables.length; d++ )
			{
				_drawables[d].nextstep()
			}
			for( var d = 0; d<_drawables.length-1; d++ )
			{
				for( var d2 = d+1; d2<_drawables.length; d2++ )
				{
					dotick(_drawables[d],_drawables[d2]);
				}
			}

			for( var d = 0; d<_drawables.length; d++ )
			{
				_drawables[d].move()
			}

		}

		if( lastmark < 0 )
			lastmark =mark;
		viewport.tick( keys );




		for( var d = 0; d<_drawables.length; d++ )
		{
			_drawables[d].tick((mark - lastmark)/step_time )
		}


		for( var d2 = 0; d2<_drawables.length; d2++ )
		{
			viewport.draw( context, width,height, _drawables[d2] );
		}
		viewport.draw( context, width,height,mid);
		viewport.drawguides( context, width, height );
	};
};

