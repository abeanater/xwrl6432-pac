#[doc = "Register `ICPDCLR` reader"]
pub type R = crate::R<IcpdclrSpec>;
#[doc = "Register `ICPDCLR` writer"]
pub type W = crate::W<IcpdclrSpec>;
#[doc = "Field `PDCLR0` reader - 0:0\\]
Used to clear PDOUT\\[0\\]
bit which corresponds to the SCL pin. Reads: Reads should return 0. User documentation should say reads are indeterminate. Writes: 0 = no effect 1 = PDOUT\\[0\\]
bit is cleared to logic low."]
pub type Pdclr0R = crate::BitReader;
#[doc = "Field `PDCLR0` writer - 0:0\\]
Used to clear PDOUT\\[0\\]
bit which corresponds to the SCL pin. Reads: Reads should return 0. User documentation should say reads are indeterminate. Writes: 0 = no effect 1 = PDOUT\\[0\\]
bit is cleared to logic low."]
pub type Pdclr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDCLR1` reader - 1:1\\]
Used to clear PDOUT\\[1\\]
bit which corresponds to the SDA pin. Reads: Reads should return 0. User documentation should say reads are indeterminate. Writes: 0 = no effect 1 = PDOUT\\[1\\]
bit is cleared to logic low."]
pub type Pdclr1R = crate::BitReader;
#[doc = "Field `PDCLR1` writer - 1:1\\]
Used to clear PDOUT\\[1\\]
bit which corresponds to the SDA pin. Reads: Reads should return 0. User documentation should say reads are indeterminate. Writes: 0 = no effect 1 = PDOUT\\[1\\]
bit is cleared to logic low."]
pub type Pdclr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU` reader - 31:2\\]
Reserved"]
pub type NuR = crate::FieldReader<u32>;
#[doc = "Field `NU` writer - 31:2\\]
Reserved"]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Used to clear PDOUT\\[0\\]
bit which corresponds to the SCL pin. Reads: Reads should return 0. User documentation should say reads are indeterminate. Writes: 0 = no effect 1 = PDOUT\\[0\\]
bit is cleared to logic low."]
    #[inline(always)]
    pub fn pdclr0(&self) -> Pdclr0R {
        Pdclr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Used to clear PDOUT\\[1\\]
bit which corresponds to the SDA pin. Reads: Reads should return 0. User documentation should say reads are indeterminate. Writes: 0 = no effect 1 = PDOUT\\[1\\]
bit is cleared to logic low."]
    #[inline(always)]
    pub fn pdclr1(&self) -> Pdclr1R {
        Pdclr1R::new(((self.bits >> 1) & 1) != 0)
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
Used to clear PDOUT\\[0\\]
bit which corresponds to the SCL pin. Reads: Reads should return 0. User documentation should say reads are indeterminate. Writes: 0 = no effect 1 = PDOUT\\[0\\]
bit is cleared to logic low."]
    #[inline(always)]
    #[must_use]
    pub fn pdclr0(&mut self) -> Pdclr0W<IcpdclrSpec> {
        Pdclr0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Used to clear PDOUT\\[1\\]
bit which corresponds to the SDA pin. Reads: Reads should return 0. User documentation should say reads are indeterminate. Writes: 0 = no effect 1 = PDOUT\\[1\\]
bit is cleared to logic low."]
    #[inline(always)]
    #[must_use]
    pub fn pdclr1(&mut self) -> Pdclr1W<IcpdclrSpec> {
        Pdclr1W::new(self, 1)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<IcpdclrSpec> {
        NuW::new(self, 2)
    }
}
#[doc = "I2C Pin Data Clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`icpdclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icpdclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcpdclrSpec;
impl crate::RegisterSpec for IcpdclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icpdclr::R`](R) reader structure"]
impl crate::Readable for IcpdclrSpec {}
#[doc = "`write(|w| ..)` method takes [`icpdclr::W`](W) writer structure"]
impl crate::Writable for IcpdclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICPDCLR to value 0"]
impl crate::Resettable for IcpdclrSpec {
    const RESET_VALUE: u32 = 0;
}
