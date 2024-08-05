#[doc = "Register `HL_REV` reader"]
pub type R = crate::R<HlRevSpec>;
#[doc = "Register `HL_REV` writer"]
pub type W = crate::W<HlRevSpec>;
#[doc = "Field `Y_MINOR` reader - 5:0\\]
Minor Revision \\[Y\\]
maintained by IP specification owner Y changes ONLY when: \\[1\\]
Features are scaled \\[up or down\\]
Flexibility exists in that this feature scalability may either be represented in the Y change or a specific register in the IP that indicates which features are exactly available \\[2\\]
When feature creeps from Is-Not list to Is list But this may not be the case once it sees silicon; in which case X will change Y does NOT change due to: \\[1\\]
Bug fixes \\[2\\]
Typos or clarifications \\[3\\]
major functional/feature change/addition/deletion Instead these changes may be reflected via R S X as applicable Spec owner maintains a customer-invisible number 'S' which changes due to: \\[1\\]
Typos/clarifications \\[2\\]
Bug documentation Note that this bug is not due to a spec change but due to implementation Nevertheless the spec tracks the IP bugs An RTL release \\[say for silicon PG11\\]
that occurs due to bug fix should document the corresponding spec number \\[XYS\\]
in its release notes - (RO )"]
pub type YMinorR = crate::FieldReader;
#[doc = "Field `Y_MINOR` writer - 5:0\\]
Minor Revision \\[Y\\]
maintained by IP specification owner Y changes ONLY when: \\[1\\]
Features are scaled \\[up or down\\]
Flexibility exists in that this feature scalability may either be represented in the Y change or a specific register in the IP that indicates which features are exactly available \\[2\\]
When feature creeps from Is-Not list to Is list But this may not be the case once it sees silicon; in which case X will change Y does NOT change due to: \\[1\\]
Bug fixes \\[2\\]
Typos or clarifications \\[3\\]
major functional/feature change/addition/deletion Instead these changes may be reflected via R S X as applicable Spec owner maintains a customer-invisible number 'S' which changes due to: \\[1\\]
Typos/clarifications \\[2\\]
Bug documentation Note that this bug is not due to a spec change but due to implementation Nevertheless the spec tracks the IP bugs An RTL release \\[say for silicon PG11\\]
that occurs due to bug fix should document the corresponding spec number \\[XYS\\]
in its release notes - (RO )"]
pub type YMinorW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CUSTOM` reader - 7:6\\]
Indicates a special version for a particular device Consequence of use may avoid use of standard Chip Support Library \\[CSL\\]
/ Drivers - (RO )"]
pub type CustomR = crate::FieldReader;
#[doc = "Field `CUSTOM` writer - 7:6\\]
Indicates a special version for a particular device Consequence of use may avoid use of standard Chip Support Library \\[CSL\\]
/ Drivers - (RO )"]
pub type CustomW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `X_MAJOR` reader - 10:8\\]
Major Revision \\[X\\]
maintained by IP specification owner X changes ONLY when: \\[1\\]
There is a major feature addition An example would be adding Master Mode to Utopia Level2 The Func field \\[or Class/Type in old PID format\\]
will remain the same X does NOT change due to: \\[1\\]
Bug fixes \\[2\\]
Change in feature parameters - (RO )"]
pub type XMajorR = crate::FieldReader;
#[doc = "Field `X_MAJOR` writer - 10:8\\]
Major Revision \\[X\\]
maintained by IP specification owner X changes ONLY when: \\[1\\]
There is a major feature addition An example would be adding Master Mode to Utopia Level2 The Func field \\[or Class/Type in old PID format\\]
will remain the same X does NOT change due to: \\[1\\]
Bug fixes \\[2\\]
Change in feature parameters - (RO )"]
pub type XMajorW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `R_RTL` reader - 15:11\\]
RTL Version \\[R\\]
maintained by IP design owner RTL follows a numbering such as XYRZ which are explained in this table R changes ONLY when: \\[1\\]
PDS uploads occur which may have been due to spec changes \\[2\\]
Bug fixes occur \\[3\\]
Resets to '0' when X or Y changes Design team has an internal 'Z' \\[customer invisible\\]
number which increments on every drop that happens due to DV and RTL updates Z resets to 0 when R increments - (RO )"]
pub type RRtlR = crate::FieldReader;
#[doc = "Field `R_RTL` writer - 15:11\\]
RTL Version \\[R\\]
maintained by IP design owner RTL follows a numbering such as XYRZ which are explained in this table R changes ONLY when: \\[1\\]
PDS uploads occur which may have been due to spec changes \\[2\\]
Bug fixes occur \\[3\\]
Resets to '0' when X or Y changes Design team has an internal 'Z' \\[customer invisible\\]
number which increments on every drop that happens due to DV and RTL updates Z resets to 0 when R increments - (RO )"]
pub type RRtlW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `FUNC` reader - 27:16\\]
Function indicates a software compatible module family If there is no level of software compatibility a new Func number \\[and hence REVISION\\]
should be assigned - (RO )"]
pub type FuncR = crate::FieldReader<u16>;
#[doc = "Field `FUNC` writer - 27:16\\]
Function indicates a software compatible module family If there is no level of software compatibility a new Func number \\[and hence REVISION\\]
should be assigned - (RO )"]
pub type FuncW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `RSVD` reader - 29:28\\]
Reserved These bits are initialized to zero and writes to them are ignored - (RO )"]
pub type RsvdR = crate::FieldReader;
#[doc = "Field `RSVD` writer - 29:28\\]
Reserved These bits are initialized to zero and writes to them are ignored - (RO )"]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCHEME` reader - 31:30\\]
Used to distinguish between old scheme and current - (RO )"]
pub type SchemeR = crate::FieldReader;
#[doc = "Field `SCHEME` writer - 31:30\\]
Used to distinguish between old scheme and current - (RO )"]
pub type SchemeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Minor Revision \\[Y\\]
maintained by IP specification owner Y changes ONLY when: \\[1\\]
Features are scaled \\[up or down\\]
Flexibility exists in that this feature scalability may either be represented in the Y change or a specific register in the IP that indicates which features are exactly available \\[2\\]
When feature creeps from Is-Not list to Is list But this may not be the case once it sees silicon; in which case X will change Y does NOT change due to: \\[1\\]
Bug fixes \\[2\\]
Typos or clarifications \\[3\\]
major functional/feature change/addition/deletion Instead these changes may be reflected via R S X as applicable Spec owner maintains a customer-invisible number 'S' which changes due to: \\[1\\]
Typos/clarifications \\[2\\]
Bug documentation Note that this bug is not due to a spec change but due to implementation Nevertheless the spec tracks the IP bugs An RTL release \\[say for silicon PG11\\]
that occurs due to bug fix should document the corresponding spec number \\[XYS\\]
in its release notes - (RO )"]
    #[inline(always)]
    pub fn y_minor(&self) -> YMinorR {
        YMinorR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Indicates a special version for a particular device Consequence of use may avoid use of standard Chip Support Library \\[CSL\\]
/ Drivers - (RO )"]
    #[inline(always)]
    pub fn custom(&self) -> CustomR {
        CustomR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Major Revision \\[X\\]
maintained by IP specification owner X changes ONLY when: \\[1\\]
There is a major feature addition An example would be adding Master Mode to Utopia Level2 The Func field \\[or Class/Type in old PID format\\]
will remain the same X does NOT change due to: \\[1\\]
Bug fixes \\[2\\]
Change in feature parameters - (RO )"]
    #[inline(always)]
    pub fn x_major(&self) -> XMajorR {
        XMajorR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
RTL Version \\[R\\]
maintained by IP design owner RTL follows a numbering such as XYRZ which are explained in this table R changes ONLY when: \\[1\\]
PDS uploads occur which may have been due to spec changes \\[2\\]
Bug fixes occur \\[3\\]
Resets to '0' when X or Y changes Design team has an internal 'Z' \\[customer invisible\\]
number which increments on every drop that happens due to DV and RTL updates Z resets to 0 when R increments - (RO )"]
    #[inline(always)]
    pub fn r_rtl(&self) -> RRtlR {
        RRtlR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Function indicates a software compatible module family If there is no level of software compatibility a new Func number \\[and hence REVISION\\]
should be assigned - (RO )"]
    #[inline(always)]
    pub fn func(&self) -> FuncR {
        FuncR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:29 - 29:28\\]
Reserved These bits are initialized to zero and writes to them are ignored - (RO )"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Used to distinguish between old scheme and current - (RO )"]
    #[inline(always)]
    pub fn scheme(&self) -> SchemeR {
        SchemeR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Minor Revision \\[Y\\]
maintained by IP specification owner Y changes ONLY when: \\[1\\]
Features are scaled \\[up or down\\]
Flexibility exists in that this feature scalability may either be represented in the Y change or a specific register in the IP that indicates which features are exactly available \\[2\\]
When feature creeps from Is-Not list to Is list But this may not be the case once it sees silicon; in which case X will change Y does NOT change due to: \\[1\\]
Bug fixes \\[2\\]
Typos or clarifications \\[3\\]
major functional/feature change/addition/deletion Instead these changes may be reflected via R S X as applicable Spec owner maintains a customer-invisible number 'S' which changes due to: \\[1\\]
Typos/clarifications \\[2\\]
Bug documentation Note that this bug is not due to a spec change but due to implementation Nevertheless the spec tracks the IP bugs An RTL release \\[say for silicon PG11\\]
that occurs due to bug fix should document the corresponding spec number \\[XYS\\]
in its release notes - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn y_minor(&mut self) -> YMinorW<HlRevSpec> {
        YMinorW::new(self, 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Indicates a special version for a particular device Consequence of use may avoid use of standard Chip Support Library \\[CSL\\]
/ Drivers - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn custom(&mut self) -> CustomW<HlRevSpec> {
        CustomW::new(self, 6)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Major Revision \\[X\\]
maintained by IP specification owner X changes ONLY when: \\[1\\]
There is a major feature addition An example would be adding Master Mode to Utopia Level2 The Func field \\[or Class/Type in old PID format\\]
will remain the same X does NOT change due to: \\[1\\]
Bug fixes \\[2\\]
Change in feature parameters - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn x_major(&mut self) -> XMajorW<HlRevSpec> {
        XMajorW::new(self, 8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
RTL Version \\[R\\]
maintained by IP design owner RTL follows a numbering such as XYRZ which are explained in this table R changes ONLY when: \\[1\\]
PDS uploads occur which may have been due to spec changes \\[2\\]
Bug fixes occur \\[3\\]
Resets to '0' when X or Y changes Design team has an internal 'Z' \\[customer invisible\\]
number which increments on every drop that happens due to DV and RTL updates Z resets to 0 when R increments - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn r_rtl(&mut self) -> RRtlW<HlRevSpec> {
        RRtlW::new(self, 11)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Function indicates a software compatible module family If there is no level of software compatibility a new Func number \\[and hence REVISION\\]
should be assigned - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn func(&mut self) -> FuncW<HlRevSpec> {
        FuncW::new(self, 16)
    }
    #[doc = "Bits 28:29 - 29:28\\]
Reserved These bits are initialized to zero and writes to them are ignored - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn rsvd(&mut self) -> RsvdW<HlRevSpec> {
        RsvdW::new(self, 28)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Used to distinguish between old scheme and current - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn scheme(&mut self) -> SchemeW<HlRevSpec> {
        SchemeW::new(self, 30)
    }
}
#[doc = "IP Revision Identifier (X.Y.R) Used by software to track features bugs and compatibility\n\nYou can [`read`](crate::Reg::read) this register and get [`hl_rev::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hl_rev::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HlRevSpec;
impl crate::RegisterSpec for HlRevSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hl_rev::R`](R) reader structure"]
impl crate::Readable for HlRevSpec {}
#[doc = "`write(|w| ..)` method takes [`hl_rev::W`](W) writer structure"]
impl crate::Writable for HlRevSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HL_REV to value 0"]
impl crate::Resettable for HlRevSpec {
    const RESET_VALUE: u32 = 0;
}
