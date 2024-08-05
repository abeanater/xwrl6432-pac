#[doc = "Register `ICPDIN` reader"]
pub type R = crate::R<IcpdinSpec>;
#[doc = "Register `ICPDIN` writer"]
pub type W = crate::W<IcpdinSpec>;
#[doc = "Field `PDIN0` reader - 0:0\\]
Indicates the logic level present on the SCL pin. Reads: 0 = Logic low present at SCL pin regardless of PFUNC setting. 1 = Logic high present at SCL pin regardless of PFUNC setting. Writes: Writes have no effect - (RW )"]
pub type Pdin0R = crate::BitReader;
#[doc = "Field `PDIN0` writer - 0:0\\]
Indicates the logic level present on the SCL pin. Reads: 0 = Logic low present at SCL pin regardless of PFUNC setting. 1 = Logic high present at SCL pin regardless of PFUNC setting. Writes: Writes have no effect - (RW )"]
pub type Pdin0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDIN1` reader - 1:1\\]
Indicates the logic level present on the SDA pin. Reads: 0 = Logic low present at SDA pin regardless of PFUNC setting. 1 = Logic high present at SDA pin regardless of PFUNC setting. Writes: Writes have no effect. - (RW )"]
pub type Pdin1R = crate::BitReader;
#[doc = "Field `PDIN1` writer - 1:1\\]
Indicates the logic level present on the SDA pin. Reads: 0 = Logic low present at SDA pin regardless of PFUNC setting. 1 = Logic high present at SDA pin regardless of PFUNC setting. Writes: Writes have no effect. - (RW )"]
pub type Pdin1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU` reader - 31:2\\]
Reserved"]
pub type NuR = crate::FieldReader<u32>;
#[doc = "Field `NU` writer - 31:2\\]
Reserved"]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates the logic level present on the SCL pin. Reads: 0 = Logic low present at SCL pin regardless of PFUNC setting. 1 = Logic high present at SCL pin regardless of PFUNC setting. Writes: Writes have no effect - (RW )"]
    #[inline(always)]
    pub fn pdin0(&self) -> Pdin0R {
        Pdin0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates the logic level present on the SDA pin. Reads: 0 = Logic low present at SDA pin regardless of PFUNC setting. 1 = Logic high present at SDA pin regardless of PFUNC setting. Writes: Writes have no effect. - (RW )"]
    #[inline(always)]
    pub fn pdin1(&self) -> Pdin1R {
        Pdin1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Reserved"]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Indicates the logic level present on the SCL pin. Reads: 0 = Logic low present at SCL pin regardless of PFUNC setting. 1 = Logic high present at SCL pin regardless of PFUNC setting. Writes: Writes have no effect - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn pdin0(&mut self) -> Pdin0W<IcpdinSpec> {
        Pdin0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates the logic level present on the SDA pin. Reads: 0 = Logic low present at SDA pin regardless of PFUNC setting. 1 = Logic high present at SDA pin regardless of PFUNC setting. Writes: Writes have no effect. - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn pdin1(&mut self) -> Pdin1W<IcpdinSpec> {
        Pdin1W::new(self, 1)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<IcpdinSpec> {
        NuW::new(self, 2)
    }
}
#[doc = "I2C Pin Data In register\n\nYou can [`read`](crate::Reg::read) this register and get [`icpdin::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icpdin::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcpdinSpec;
impl crate::RegisterSpec for IcpdinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icpdin::R`](R) reader structure"]
impl crate::Readable for IcpdinSpec {}
#[doc = "`write(|w| ..)` method takes [`icpdin::W`](W) writer structure"]
impl crate::Writable for IcpdinSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICPDIN to value 0"]
impl crate::Resettable for IcpdinSpec {
    const RESET_VALUE: u32 = 0;
}
