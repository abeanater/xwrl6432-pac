#[doc = "Register `QUEPRI` reader"]
pub type R = crate::R<QuepriSpec>;
#[doc = "Register `QUEPRI` writer"]
pub type W = crate::W<QuepriSpec>;
#[doc = "Field `PRIQ0` reader - 2:0\\]
Priority Level for Queue 0 Dictates the priority level used for the OPTIONS field programmation for Qn TRs. Sets the priority used for TC read and write commands."]
pub type Priq0R = crate::FieldReader;
#[doc = "Field `PRIQ0` writer - 2:0\\]
Priority Level for Queue 0 Dictates the priority level used for the OPTIONS field programmation for Qn TRs. Sets the priority used for TC read and write commands."]
pub type Priq0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RES30` reader - 3:3\\]
RESERVE FIELD"]
pub type Res30R = crate::BitReader;
#[doc = "Field `RES30` writer - 3:3\\]
RESERVE FIELD"]
pub type Res30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIQ1` reader - 6:4\\]
Priority Level for Queue 1 Dictates the priority level used for the OPTIONS field programmation for Qn TRs. Sets the priority used for TC read and write commands."]
pub type Priq1R = crate::FieldReader;
#[doc = "Field `PRIQ1` writer - 6:4\\]
Priority Level for Queue 1 Dictates the priority level used for the OPTIONS field programmation for Qn TRs. Sets the priority used for TC read and write commands."]
pub type Priq1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RES29` reader - 31:7\\]
RESERVE FIELD"]
pub type Res29R = crate::FieldReader<u32>;
#[doc = "Field `RES29` writer - 31:7\\]
RESERVE FIELD"]
pub type Res29W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Priority Level for Queue 0 Dictates the priority level used for the OPTIONS field programmation for Qn TRs. Sets the priority used for TC read and write commands."]
    #[inline(always)]
    pub fn priq0(&self) -> Priq0R {
        Priq0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res30(&self) -> Res30R {
        Res30R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Priority Level for Queue 1 Dictates the priority level used for the OPTIONS field programmation for Qn TRs. Sets the priority used for TC read and write commands."]
    #[inline(always)]
    pub fn priq1(&self) -> Priq1R {
        Priq1R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:31 - 31:7\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res29(&self) -> Res29R {
        Res29R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Priority Level for Queue 0 Dictates the priority level used for the OPTIONS field programmation for Qn TRs. Sets the priority used for TC read and write commands."]
    #[inline(always)]
    #[must_use]
    pub fn priq0(&mut self) -> Priq0W<QuepriSpec> {
        Priq0W::new(self, 0)
    }
    #[doc = "Bit 3 - 3:3\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res30(&mut self) -> Res30W<QuepriSpec> {
        Res30W::new(self, 3)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Priority Level for Queue 1 Dictates the priority level used for the OPTIONS field programmation for Qn TRs. Sets the priority used for TC read and write commands."]
    #[inline(always)]
    #[must_use]
    pub fn priq1(&mut self) -> Priq1W<QuepriSpec> {
        Priq1W::new(self, 4)
    }
    #[doc = "Bits 7:31 - 31:7\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res29(&mut self) -> Res29W<QuepriSpec> {
        Res29W::new(self, 7)
    }
}
#[doc = "Queue Priority\n\nYou can [`read`](crate::Reg::read) this register and get [`quepri::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`quepri::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QuepriSpec;
impl crate::RegisterSpec for QuepriSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`quepri::R`](R) reader structure"]
impl crate::Readable for QuepriSpec {}
#[doc = "`write(|w| ..)` method takes [`quepri::W`](W) writer structure"]
impl crate::Writable for QuepriSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QUEPRI to value 0"]
impl crate::Resettable for QuepriSpec {
    const RESET_VALUE: u32 = 0;
}
